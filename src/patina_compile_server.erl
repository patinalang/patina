-module(patina_compile_server).

-export([start_link/1]).

-behaviour(gen_server).
-export([init/1, handle_call/3, handle_cast/2, handle_continue/2]).

-define(ACCEPT_TIMEOUT, timer:seconds(30)).
-define(MACRO_TIMEOUT_SECONDS, 30).
-define(MACRO_TIMEOUT, timer:seconds(?MACRO_TIMEOUT_SECONDS)).

start_link(SockPath) ->
    gen_server:start_link(?MODULE, [SockPath], []).

init([SockPath]) ->
    {ok, Socket} = socket:open(local, stream, default, #{}),
    ok = socket:bind(Socket, #{family => local, path => SockPath}),
    ok = socket:listen(Socket),
    {ok, Socket, {continue, accept}}.

handle_call(_Request, _From, Socket) ->
    {reply, ok, Socket}.

handle_cast(_Msg, Socket) ->
    {noreply, Socket}.

handle_continue(accept, Socket) ->
    Deadline = erlang:monotonic_time(millisecond) + ?ACCEPT_TIMEOUT,
    case accept_client(Socket, Deadline) of
        {ok, ClientSock} ->
            socket_loop(ClientSock),
            socket:close(ClientSock);
        {error, Reason} ->
            io:format(standard_error, "patina compile server cannot accept: ~tp~n", [Reason])
    end,
    erlang:halt(0).

accept_client(Sock, Deadline) ->
    case Deadline - erlang:monotonic_time(millisecond) of
        Remaining when Remaining =< 0 ->
            {error, timeout};
        Remaining ->
            case socket:accept(Sock, Remaining) of
                {ok, ClientSock} ->
                    {ok, ClientSock};
                {error, timeout} ->
                    {error, timeout};
                {error, Reason} ->
                    timer:sleep(1),
                    case Deadline - erlang:monotonic_time(millisecond) =< 0 of
                        true -> {error, Reason};
                        false -> accept_client(Sock, Deadline)
                    end
            end
    end.

socket_loop(Sock) ->
    case read_msg(Sock) of
        {ok, [Op | Args]} ->
            handle_request(Sock, Op, Args),
            socket_loop(Sock);
        closed ->
            ok
    end.

handle_request(Sock, Op, Args) ->
    try
        do_request(Sock, Op, Args)
    catch
        Class:Reason:Stacktrace ->
            write_msg(Sock, [~"raise", Class, Reason, Stacktrace])
    end.

do_request(Sock, ~"compile", [Eaf]) ->
    case compile_forms(binary_to_term(Eaf)) of
        {ok, _Mod, Beam, Warnings} ->
            write_msg(Sock, [~"ok", Beam, Warnings]);
        {error, Errors, Warnings} ->
            write_msg(Sock, [~"error", Errors, Warnings])
    end;
do_request(Sock, ~"abstr", [Eaf]) ->
    Rendered = unicode:characters_to_binary(io_lib:format("~tp", [binary_to_term(Eaf)])),
    write_msg(Sock, [~"ok", Rendered]);
do_request(Sock, ~"erl", [Eaf]) ->
    Forms = binary_to_term(Eaf),
    Rendered = unicode:characters_to_binary([erl_prettypr:format(erl_syntax:form_list(Forms)), "\n"]),
    write_msg(Sock, [~"ok", Rendered]);
do_request(Sock, ~"code_path", Dirs) ->
    lists:foreach(fun(Dir) -> code:add_pathz(binary_to_list(Dir)) end, Dirs),
    write_msg(Sock, [~"ok"]);
do_request(Sock, ~"echo", [Bin]) ->
    write_msg(Sock, [~"ok", term_to_binary(binary_to_term(Bin), [{minor_version, 2}])]);
do_request(Sock, ~"macro", [Mod, Fun | ArgBins]) ->
    Args = [binary_to_term(Arg) || Arg <- ArgBins],
    Module = binary_to_atom(Mod, utf8),
    Function = binary_to_atom(Fun, utf8),
    run_macro(Sock, Module, Function, Args).

run_macro(Sock, Module, Function, Args) ->
    {Pid, Ref} = spawn_opt(
        fun() ->
            Parent =
                receive
                    {reply_to, Parent0} -> Parent0
                end,

            Result =
                try
                    {ok, apply(Module, Function, Args)}
                catch
                    Class:Reason:Stack ->
                        FilteredStack = lists:takewhile(
                            fun
                                ({?MODULE, _, _, _}) -> false;
                                (_) -> true
                            end,
                            Stack
                        ),
                        {error, {Class, Reason, FilteredStack}}
                end,
            Parent ! {Parent, Result}
        end,
        [{monitor, [{alias, reply_demonitor}]}]
    ),

    Pid ! {reply_to, Ref},

    Result =
        case receive_run_macro_result(Ref) of
            {error, timeout} ->
                exit(Pid, kill),
                case receive_run_macro_result(Ref) of
                    {error, {down, killed}} -> {error, timeout};
                    Other -> Other
                end;
            Other ->
                Other
        end,

    case Result of
        {ok, Tokens} ->
            write_msg(Sock, [~"ok", term_to_binary(Tokens, [{minor_version, 2}])]);
        {error, {Class, Reason, Stack}} ->
            write_msg(Sock, [~"error", Class, Reason, Stack]);
        {error, {down, Reason}} ->
            write_msg(Sock, [~"down", Reason]);
        {error, timeout} ->
            write_msg(Sock, [~"timeout"])
    end.

receive_run_macro_result(Ref) ->
    receive
        {Ref, {ok, Result}} ->
            {ok, Result};
        {Ref, {error, {Class, Reason, Stack}}} ->
            {error, {Class, Reason, Stack}};
        {'DOWN', Ref, process, _Pid, Reason} ->
            {error, {down, Reason}}
    after ?MACRO_TIMEOUT ->
        {error, timeout}
    end.

compile_forms(Forms) ->
    CompileOpts = [
        binary, return, nowarn_opportunistic, nowarn_underscore_match, nowarn_shadow_vars
    ],
    %% We stop at `to_core0` specifically, so we can rewrite forms before lambdas get inlined.
    case compile:noenv_forms(Forms, [to_core0 | CompileOpts]) of
        {ok, _Mod, Core0, Warnings0} ->
            Core1 = core_transform(Core0),
            case compile:noenv_forms(Core1, [from_core | CompileOpts]) of
                {ok, Mod, Beam, Warnings1} ->
                    {ok, Mod, Beam, lists:uniq(Warnings0 ++ Warnings1)};
                {error, Errors, Warnings1} ->
                    {error, Errors, lists:uniq(Warnings0 ++ Warnings1)}
            end;
        {error, Errors, Warnings} ->
            {error, Errors, Warnings}
    end.

read_msg(Sock) ->
    case socket:recv(Sock, 1) of
        {ok, <<Len:8/unsigned-integer>>} ->
            {ok, lists:map(fun(_) -> read_bin(Sock) end, lists:seq(1, Len))};
        {error, _} ->
            closed
    end.

read_bin(Sock) ->
    {ok, <<Len:32/unsigned-integer>>} = socket:recv(Sock, 4),
    {ok, Data} = socket:recv(Sock, Len),
    Data.

write_msg(Sock, [_ | _] = Msg) ->
    Len = length(Msg),
    socket:send(Sock, <<Len:8/unsigned-integer>>),
    lists:foreach(fun(Data) -> write(Sock, Data) end, Msg).

write(Sock, <<Data/binary>>) ->
    Len = byte_size(Data),
    socket:send(Sock, <<Len:32/unsigned-integer>>),
    socket:send(Sock, Data);
write(Sock, Data) ->
    Bin = unicode:characters_to_binary(io_lib:format("~tp", [Data])),
    write(Sock, Bin).

%% -----------------------------------------------------------------------------
%% Control markers
%% -----------------------------------------------------------------------------

core_transform(Module0) ->
    Module1 = cerl_trees:map(fun rewrite_labels/1, Module0),
    cerl_trees:map(fun rewrite/1, gather(Module1)).

%% Rewrites labels into `letrec_goto` join points. A label is scoped to one `case`, and jumps
%% never cross a `try` or a function boundary.
%%
%% - A `case` clause matching `{'$patina_label_N', ...Vars}` is rewritten into `letrec` that
%%   `'$patina_marker':jump('$patina_label_n', ...Vars)` jumps to.
%%
%% - `'$patina_marker:jump(Label, Args...)` jumps to `Label/N`.
%%
%% What's left should be just a case with a single `_ -> ...` clause that points to "real" body
%% that uses the jumps.
rewrite_labels(Expr) ->
    case cerl:type(Expr) of
        'case' -> rewrite_join_case(Expr);
        call -> rewrite_label_jump(Expr);
        seq -> rewrite_label_seq(Expr);
        _ -> Expr
    end.

rewrite_join_case(Case) ->
    {Clauses, Joins} = lists:foldl(fun rewrite_label_clause/2, {[], []}, cerl:case_clauses(Case)),
    lists:foldl(
        fun(Join, Acc) -> cerl:ann_c_letrec([letrec_goto], [Join], Acc) end,
        cerl:update_c_case(Case, cerl:case_arg(Case), lists:reverse(Clauses)),
        Joins
    ).

rewrite_label_clause(Clause, {Clauses, Joins}) ->
    case label_clause(Clause) of
        {Label, Vars} ->
            Name = cerl:c_fname(cerl:atom_val(Label), length(Vars)),
            Body = cerl:clause_body(Clause),
            Join = {Name, cerl:c_fun(Vars, Body)},
            {Clauses, [Join | Joins]};
        false ->
            {[Clause | Clauses], Joins}
    end.

label_clause(Clause) ->
    maybe
        [Pat] ?= cerl:clause_pats(Clause),
        true ?= cerl:is_c_tuple(Pat),
        [Label | Vars] ?= cerl:tuple_es(Pat),
        true ?= cerl:is_c_atom(Label),
        true ?= is_label(cerl:atom_val(Label)),
        {Label, Vars}
    else
        _ -> false
    end.

rewrite_label_jump(Call) ->
    IsJump =
        cerl:is_c_atom(cerl:call_module(Call)) andalso
            cerl:atom_val(cerl:call_module(Call)) =:= '$patina_marker' andalso
            cerl:concrete(cerl:call_name(Call)) =:= jump,

    case IsJump of
        true ->
            [Label | Args] = cerl:call_args(Call),
            cerl:c_apply(cerl:c_fname(cerl:atom_val(Label), length(Args)), Args);
        false ->
            Call
    end.

%% `do Jump Rest`: `Rest` cannot run.
rewrite_label_seq(Seq) ->
    Arg = cerl:seq_arg(Seq),
    IsJump =
        cerl:is_c_apply(Arg) andalso cerl:is_c_fname(cerl:apply_op(Arg)) andalso
            is_label(cerl:fname_id(cerl:apply_op(Arg))),

    case IsJump of
        true -> Arg;
        false -> Seq
    end.

is_label(Name) ->
    lists:prefix("$patina_label_", atom_to_list(Name)).

%% Adds annotations to the module.
%% `MarkerJoins` is an ordset of `{MarkerTag, Arity}`.
%%
%% - `{{patina, jump_targets}, MarkerJoins}`: on an expression, these are the join points *outside*
%%   of the expression that are targeted by the jumps from *within* the expression. In other words,
%%   these are the outside join points that are "needed" (or targeted) by its jumps.
%%   This annotation is only used within `gather`; it's a helper to compute the other annotations.
%%
%% - `{{patina, defined_joins}, MarkerJoins}`: on a fun, the join points targeted by the jumps
%%   in its body. While `jump_targets` says "what join points are needed outside
%%   of this expression", this says "what join points must I define around this body".
%%
%% - `{{patina, dispatch}, MarkerJoins}`: on a try or a local function call, when we create
%%   the dispatching `case` that peels marker tuples, the join points targeted by those
%%   `case` branches.
%%
%% - `{{patina, local}, Name}`: on a local function. A local function comes from a letrec that is
%%   not a join point, and isn't a lambda (no `id` annotation). Whether a function is local,
%%   module-level, or a letrec_goto body can only be determined from parent's point of view,
%%   so we calculate that in preorder for later use in postorder. Local functions (like `try` exprs)
%%   are special, because they need to return a marker and can't just jump out.
%%
%% - `{{patina, function}, Name}`: on a module-level function. Like `local`, it is marked in
%%   preorder from the parent (the module). `rewrite` uses `Name` for the `'$patina_continue'`
%%   self-call.
%%
%% - `{{patina, lambda}, Name}`: on a named lambda fun. Like `function`, `rewrite` uses `Name`
%%   for the `'$patina_continue'` self-call.
gather(Module0) ->
    {Module, _Locals} = cerl_trees:mapfold(fun gather_pre/2, fun gather_post/2, #{}, Module0),
    Module.

gather_pre(Expr, Locals) ->
    case cerl:type(Expr) of
        module -> {annotate_functions(Expr), Locals};
        letrec -> {annotate_locals(Expr), Locals};
        _ -> {Expr, Locals}
    end.

annotate_functions(Module) ->
    Defs = lists:map(
        fun({Name, Fun}) -> {Name, put_annotation(function, cerl:var_name(Name), Fun)} end,
        cerl:module_defs(Module)
    ),
    cerl:update_c_module(
        Module,
        cerl:module_name(Module),
        cerl:module_exports(Module),
        cerl:module_attrs(Module),
        Defs
    ).

annotate_locals(Letrec) ->
    case lists:member(letrec_goto, cerl:get_ann(Letrec)) of
        true ->
            Letrec;
        false ->
            Defs = lists:map(
                fun({Name, Fun}) ->
                    case is_lambda(Fun) of
                        true -> {Name, put_annotation(lambda, cerl:var_name(Name), Fun)};
                        false -> {Name, put_annotation(local, cerl:var_name(Name), Fun)}
                    end
                end,
                cerl:letrec_defs(Letrec)
            ),
            cerl:update_c_letrec(Letrec, Defs, cerl:letrec_body(Letrec))
    end.

gather_post(Expr, Locals) ->
    case cerl:type(Expr) of
        'fun' -> gather_post_fun(Expr, Locals);
        _ -> gather_post_expr(Expr, Locals)
    end.

gather_post_fun(Fun0, Locals) ->
    MarkerJoins = get_annotation(jump_targets, cerl:fun_body(Fun0), []),
    Fun = put_annotation(defined_joins, MarkerJoins, Fun0),
    case get_annotation(local, Fun) of
        undefined ->
            case is_lambda(Fun) of
                true -> {Fun, Locals};
                false -> {put_annotation(jump_targets, MarkerJoins, Fun), Locals}
            end;
        Name ->
            {Fun, Locals#{Name => MarkerJoins}}
    end.

gather_post_expr(Expr0, Locals) ->
    Dispatch = dispatch_marker_joins(Expr0, Locals),
    Own =
        case marker_tuple(Expr0) of
            {MarkerTag, Args} -> [{MarkerTag, length(Args)}];
            false -> []
        end,
    Children = [
        get_annotation(jump_targets, Child, [])
     || Group <- cerl:subtrees(Expr0), Child <- Group
    ],
    MarkerJoins = ordsets:union([Own, Dispatch | Children]),
    Expr1 = put_annotation(dispatch, Dispatch, Expr0),
    Expr = put_annotation(jump_targets, MarkerJoins, Expr1),
    {Expr, Locals}.

dispatch_marker_joins(Expr, Locals) ->
    case cerl:type(Expr) of
        'try' ->
            get_annotation(jump_targets, cerl:try_arg(Expr), []);
        apply ->
            Op = cerl:apply_op(Expr),
            case cerl:is_c_fname(Op) of
                true -> maps:get(cerl:var_name(Op), Locals, []);
                false -> []
            end;
        _ ->
            []
    end.

%% Rewrites the '$patina_marker' control-flow markers into jumps.
%%
%% A jump is an apply of a `letrec_goto` join point. This transform is meant to run before
%% any optimization, because the inlining opt can inline a lambda with `return` into parent body.
%%
%% - `return(Value)` and `break(Value)` jump to `'$patina_return'/1`.
%% - `continue(Args...)` jumps to `'$patina_continue'/N`.
%%
%% A jump goes to the innermost join point with its name. The labels are always named the same
%% (as above), so they follow lexical shadowing.
%%
%% The same jump has a different effect in each scope:
%%
%% - If the marker is inside a local function (e.g. an outlined `after` block), it can't jump
%%   to the outer function, so it returns the marker as a tuple `{'$patina_return', Value}`
%%   or `{$patina_continue, ..Args}`.
%%
%% - If the marker is inside `try` protected expression it also can't jump outside of `try` block,
%%   so it likewise returns the tuple from the block.
%%
%% - At call sites for both of the scenarios above, we have to have a `case` that peels out markers,
%%   and jumps again, starting this list from the top.
%%
%% - In a (non-local) function, a `continue` jumps to a tail-call-helper that calls self-function
%%   again.
%%
%% - In a (non-local) function, a `return` or `break` simply `ret`.
rewrite(Expr0) ->
    Expr = drop_gather_annotations(Expr0),
    case cerl:type(Expr) of
        call -> rewrite_call(Expr);
        seq -> rewrite_seq(Expr);
        apply -> rewrite_apply(Expr0, Expr);
        'try' -> rewrite_try(Expr0, Expr);
        'fun' -> rewrite_fun(Expr0, Expr);
        _ -> Expr
    end.

rewrite_call(Expr) ->
    case marker_tuple(Expr) of
        {MarkerTag, Args} -> cerl:c_apply(cerl:c_fname(MarkerTag, length(Args)), Args);
        false -> Expr
    end.

rewrite_seq(Expr) ->
    case is_marker_jump(cerl:seq_arg(Expr)) of
        %% `do Jump Rest`: `Rest` cannot run.
        true -> cerl:seq_arg(Expr);
        false -> Expr
    end.

rewrite_apply(Expr0, Expr) ->
    dispatch_on_marker_tuple(Expr, get_annotation(dispatch, Expr0, [])).

rewrite_try(Expr0, Expr) ->
    MarkerJoins = get_annotation(dispatch, Expr0, []),
    Try = cerl:update_c_try(
        Expr,
        add_forwarding_join_points(cerl:try_arg(Expr), MarkerJoins),
        cerl:try_vars(Expr),
        cerl:try_body(Expr),
        cerl:try_evars(Expr),
        cerl:try_handler(Expr)
    ),
    dispatch_on_marker_tuple(Try, MarkerJoins).

rewrite_fun(Expr0, Expr) ->
    MarkerJoins = get_annotation(defined_joins, Expr0, []),
    Function = get_annotation(function, Expr0),
    Local = get_annotation(local, Expr0),
    Lambda = get_annotation(lambda, Expr0),
    if
        Function =/= undefined ->
            add_function_join_points(Expr, Function, MarkerJoins);
        Local =/= undefined ->
            cerl:update_c_fun(
                Expr,
                cerl:fun_vars(Expr),
                add_forwarding_join_points(cerl:fun_body(Expr), MarkerJoins)
            );
        true ->
            case is_lambda(Expr) of
                true -> add_function_join_points(Expr, Lambda, MarkerJoins);
                %% A `letrec_goto` label body is part of the enclosing scope.
                false -> Expr
            end
    end.

is_marker_jump(Expr) ->
    cerl:is_c_apply(Expr) andalso cerl:is_c_fname(cerl:apply_op(Expr)) andalso
        lists:member(cerl:fname_id(cerl:apply_op(Expr)), ['$patina_return', '$patina_continue']).

is_lambda(Expr) ->
    cerl:is_c_fun(Expr) andalso lists:keymember(id, 1, cerl:get_ann(Expr)).

%% Wraps (non-local) function's body with join points that exit from the function.
add_function_join_points(Fun, Self, MarkerJoins) ->
    Body = lists:foldl(
        fun
            ({'$patina_return', 1} = MarkerJoin, Acc) ->
                [Var] = arg_vars(1),
                Join = cerl:c_fun([Var], Var),
                join_point(MarkerJoin, Join, Acc);
            ({'$patina_continue', Arity} = MarkerJoin, Acc) ->
                {Name, Arity} = Self,
                Vars = arg_vars(Arity),
                Join = cerl:c_fun(Vars, cerl:c_apply(cerl:c_fname(Name, Arity), Vars)),
                join_point(MarkerJoin, Join, Acc)
        end,
        cerl:fun_body(Fun),
        MarkerJoins
    ),
    cerl:update_c_fun(Fun, cerl:fun_vars(Fun), Body).

%% Wraps (local/try) body with join points that turn the jump into a tuple tagged with a marker.
add_forwarding_join_points(Body, MarkerJoins) ->
    lists:foldl(
        fun({MarkerTag, Arity} = MarkerJoin, Acc) ->
            Vars = arg_vars(Arity),
            Tuple = cerl:c_tuple([cerl:c_atom(MarkerTag) | Vars]),
            join_point(MarkerJoin, cerl:c_fun(Vars, Tuple), Acc)
        end,
        Body,
        MarkerJoins
    ).

join_point({Name, Arity}, Fun, Body) ->
    cerl:ann_c_letrec([letrec_goto, no_inline], [{cerl:c_fname(Name, Arity), Fun}], Body).

dispatch_on_marker_tuple(Expr, []) ->
    Expr;
dispatch_on_marker_tuple(Expr, MarkerJoins) ->
    Value = cerl:c_var('@patina_value'),
    Other = cerl:c_var('@patina_other'),
    Clauses = lists:map(
        fun({MarkerTag, Arity}) ->
            Vars = arg_vars(Arity),
            cerl:c_clause(
                [cerl:c_tuple([cerl:c_atom(MarkerTag) | Vars])],
                cerl:c_apply(cerl:c_fname(MarkerTag, Arity), Vars)
            )
        end,
        MarkerJoins
    ),
    Case = cerl:c_case(Value, Clauses ++ [cerl:c_clause([Other], Other)]),
    cerl:c_let([Value], Expr, Case).

marker_tuple(Expr) ->
    IsMarker =
        cerl:is_c_call(Expr) andalso
            cerl:is_c_atom(cerl:call_module(Expr)) andalso
            cerl:atom_val(cerl:call_module(Expr)) =:= '$patina_marker',
    case IsMarker of
        true ->
            Marker =
                case cerl:concrete(cerl:call_name(Expr)) of
                    continue -> '$patina_continue';
                    return -> '$patina_return';
                    break -> '$patina_return'
                end,
            {Marker, cerl:call_args(Expr)};
        false ->
            false
    end.

put_annotation(_Key, [], Expr) -> Expr;
put_annotation(Key, Value, Expr) -> cerl:add_ann([{{patina, Key}, Value}], Expr).

get_annotation(Key, Expr) ->
    get_annotation(Key, Expr, undefined).

get_annotation(Key, Expr, Default) ->
    case lists:keyfind({patina, Key}, 1, cerl:get_ann(Expr)) of
        {{patina, Key}, Value} -> Value;
        false -> Default
    end.

drop_gather_annotations(Expr) ->
    Anns = lists:filter(
        fun
            ({{patina, _}, _}) -> false;
            (_) -> true
        end,
        cerl:get_ann(Expr)
    ),
    cerl:set_ann(Expr, Anns).

arg_vars(N) ->
    [cerl:c_var(list_to_atom("@patina_arg_" ++ integer_to_list(I))) || I <- lists:seq(1, N)].
