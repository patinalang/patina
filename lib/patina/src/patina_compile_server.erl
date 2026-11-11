-module(patina_compile_server).

-export([ensure_started/0, socket_path/0]).

-behaviour(gen_server).
-export([init/1, handle_call/3, handle_cast/2, handle_info/2, terminate/2]).

-include_lib("kernel/include/file.hrl").

-define(MACRO_TIMEOUT_SECONDS, 30).
-define(MACRO_TIMEOUT, timer:seconds(?MACRO_TIMEOUT_SECONDS)).
-define(LISTEN_BACKLOG, 1024).

-define(COMPILE_OPTS, [
    binary,
    return,
    nowarn_opportunistic,
    nowarn_underscore_match,
    nowarn_shadow_vars,
    nowarn_unused_function
]).

-doc """
Starts the server, unless it's already running.
It listens on an Unix socket created in `tmp_dir/0`.
""".
-spec ensure_started() -> {ok, pid()} | {error, term()}.
ensure_started() ->
    SockPath = filename:join(tmp_dir(), "patina-compile-" ++ os:getpid() ++ ".sock"),
    case gen_server:start({local, ?MODULE}, ?MODULE, SockPath, []) of
        {ok, Pid} -> {ok, Pid};
        {error, {already_started, Pid}} -> {ok, Pid};
        {error, Reason} -> {error, Reason}
    end.

-spec socket_path() -> file:filename_all().
socket_path() ->
    gen_server:call(?MODULE, socket_path).

-spec init(file:filename_all()) -> {ok, socket:socket()}.
init(SockPath) ->
    process_flag(trap_exit, true),

    %% A stale file from an earlier VM with the same OS pid.
    _ = file:delete(SockPath),

    {ok, Socket} = socket:open(local, stream, default, #{}),
    ok = socket:bind(Socket, #{family => local, path => SockPath}),
    ok = socket:listen(Socket, ?LISTEN_BACKLOG),

    spawn_handler(Socket),
    {ok, Socket}.

handle_call(socket_path, _From, Socket) ->
    {reply, sock_path(Socket), Socket};
handle_call(_Request, _From, State) ->
    {reply, ok, State}.

handle_cast(_Msg, State) ->
    {noreply, State}.

handle_info({accepted, _Handler}, Socket) ->
    spawn_handler(Socket),
    {noreply, Socket};
handle_info({'EXIT', _Handler, {accept_failed, _} = Reason}, Socket) ->
    {stop, Reason, Socket};
handle_info(_Msg, State) ->
    {noreply, State}.

terminate(_Reason, Socket) ->
    Path = sock_path(Socket),
    socket:close(Socket),
    _ = file:delete(Path),
    ok.

sock_path(Socket) ->
    {ok, #{path := Path}} = socket:sockname(Socket),
    Path.

spawn_handler(Socket) ->
    Parent = self(),
    proc_lib:spawn_link(fun() -> handler(Parent, Socket) end).

handler(Parent, Socket) ->
    case socket:accept(Socket) of
        {ok, ClientSock} ->
            Parent ! {accepted, self()},
            socket_loop(ClientSock);
        {error, closed} ->
            ok;
        {error, Reason} ->
            exit({accept_failed, Reason})
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
    case compile_forms(binary_to_term(Eaf), []) of
        {ok, _Mod, Beam, Warnings} ->
            write_msg(Sock, [~"ok", Beam, Warnings]);
        {error, Errors, Warnings} ->
            write_msg(Sock, [~"error", Errors, Warnings])
    end;
do_request(Sock, ~"render_term", [Term]) ->
    Rendered = unicode:characters_to_binary(io_lib:format("~tp", [binary_to_term(Term)])),
    write_msg(Sock, [~"ok", Rendered]);
do_request(Sock, ~"erl", [Eaf]) ->
    Forms = binary_to_term(Eaf),
    Rendered = unicode:characters_to_binary([erl_prettypr:format(erl_syntax:form_list(Forms)), "\n"]),
    write_msg(Sock, [~"ok", Rendered]);
do_request(Sock, <<"core", _/binary>> = Req, [Eaf]) ->
    Forms = binary_to_term(Eaf),
    Result =
        case Req of
            ~"core0" -> to_core0(Forms);
            ~"core" -> to_core(Forms)
        end,
    case Result of
        {ok, _Mod, Core, _Warnings} ->
            write_msg(Sock, [~"ok", unicode:characters_to_binary([core_pp:format(Core), "\n"])]);
        {error, Errors, Warnings} ->
            write_msg(Sock, [~"error", Errors, Warnings])
    end;
do_request(Sock, ~"code_path", Dirs) ->
    Encoding = file:native_name_encoding(),
    code:add_pathsa(lists:reverse([unicode:characters_to_list(Dir, Encoding) || Dir <- Dirs])),
    write_msg(Sock, [~"ok"]);
do_request(Sock, ~"load_modules", Files) ->
    case load_modules(Files) of
        ok -> write_msg(Sock, [~"ok"]);
        {error, Reason} -> write_msg(Sock, [~"error", Reason])
    end;
do_request(Sock, ~"otp_version", []) ->
    OtpVersion =
        case persistent_term:get({?MODULE, otp_version}, undefined) of
            undefined ->
                FetchedVersion = otp_version(),
                persistent_term:put({?MODULE, otp_version}, FetchedVersion),
                FetchedVersion;
            CachedVersion ->
                CachedVersion
        end,
    write_msg(Sock, [~"ok", OtpVersion]);
do_request(Sock, ~"echo", [Bin]) ->
    write_msg(Sock, [~"ok", term_to_binary(binary_to_term(Bin), [{minor_version, 2}])]);
do_request(Sock, ~"macro", [Mod, Fun | ArgBins]) ->
    Args = [binary_to_term(Arg) || Arg <- ArgBins],
    Module = binary_to_atom(Mod, utf8),
    Function = binary_to_atom(Fun, utf8),
    run_macro(Sock, Module, Function, Args).

%% The full OTP version, such as `29.0.1`, or else just the major release.
%% Inspired by https://github.com/fishcakez/dialyze/blob/6698ae582c77940ee10b4babe4adeff22f1b7779/lib/mix/tasks/dialyze.ex#L168
otp_version() ->
    Major = erlang:system_info(otp_release),
    VsnFile = filename:join([code:root_dir(), "releases", Major, "OTP_VERSION"]),
    OtpVersion =
        maybe
            {ok, Contents} ?= file:read_file(VsnFile),
            [Full] ?= binary:split(Contents, ~"\n", [global, trim_all]),
            Full
        else
            _ -> unicode:characters_to_binary(Major)
        end,
    OtpVersion.

load_modules(Files) ->
    Stale = lists:filtermap(fun stale_module/1, Files),
    case [Module || {Module, _File, _Beam} <- Stale, not code:soft_purge(Module)] of
        [] -> code:atomic_load(Stale);
        InUse -> {error, [{Module, old_code_in_use} || Module <- InUse]}
    end.

stale_module(FileBin) ->
    File = unicode:characters_to_list(FileBin, file:native_name_encoding()),
    Module = list_to_atom(filename:basename(File, ".beam")),
    maybe
        {file, _} ?= code:is_loaded(Module),
        {ok, Beam} ?= file:read_file(File),
        {ok, {Module, Md5}} ?= beam_lib:md5(Beam),
        false ?= Md5 =:= Module:module_info(md5),

        {true, {Module, File, Beam}}
    else
        _ -> false
    end.

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

to_core0(Forms) ->
    %% We stop at `to_core0` specifically, so we can rewrite forms before lambdas get inlined.
    case compile:noenv_forms(Forms, [to_core0 | ?COMPILE_OPTS]) of
        {ok, Mod, Core0, Warnings} ->
            {ok, Mod, core_transform(Core0), Warnings};
        {error, Errors, Warnings} ->
            {error, Errors, Warnings}
    end.

to_core(Forms) ->
    compile_forms(Forms, [to_core]).

compile_forms(Forms, Opts) ->
    case to_core0(Forms) of
        {ok, _Mod, Core0, Warnings0} ->
            case compile:noenv_forms(Core0, [from_core | Opts ++ ?COMPILE_OPTS]) of
                {ok, Mod, Result, Warnings1} ->
                    {ok, Mod, Result, lists:uniq(Warnings0 ++ Warnings1)};
                {error, Errors, Warnings1} ->
                    {error, Errors, lists:uniq(Warnings0 ++ Warnings1)}
            end;
        {error, Errors, Warnings} ->
            {error, Errors, Warnings}
    end.

read_msg(Sock) ->
    case socket:recv(Sock, 4) of
        {ok, <<Count:32/unsigned-integer>>} ->
            {ok, lists:map(fun(_) -> read_bin(Sock) end, lists:seq(1, Count))};
        {error, _} ->
            closed
    end.

read_bin(Sock) ->
    {ok, <<Len:32/unsigned-integer>>} = socket:recv(Sock, 4),
    case Len of
        0 ->
            <<>>;
        _ ->
            {ok, Data} = socket:recv(Sock, Len),
            Data
    end.

write_msg(Sock, [_ | _] = Msg) ->
    socket:send(Sock, <<(length(Msg)):32/unsigned-integer>>),
    lists:foreach(fun(Data) -> write(Sock, Data) end, Msg).

write(Sock, <<Data/binary>>) ->
    socket:send(Sock, <<(byte_size(Data)):32/unsigned-integer>>),
    socket:send(Sock, Data);
write(Sock, Data) ->
    Bin = unicode:characters_to_binary(io_lib:format("~tp", [Data])),
    write(Sock, Bin).

%% -----------------------------------------------------------------------------
%% Control markers
%% -----------------------------------------------------------------------------

core_transform(Module0) ->
    {Module, _NextLabel} = cerl_trees:label(Module0),
    Table = ets:new(?MODULE, [set, private]),
    try
        gather(Module, Table),
        cerl_trees:map(fun(Expr) -> rewrite(Expr, Table) end, Module)
    after
        ets:delete(Table)
    end.

-record(gather, {
    %% The table of gathered facts.
    table,
    %% A stack of the ordsets of labels in scope in each frame, innermost first.
    scope_labels = []
}).

%% Gathers facts about the module's nodes.
%%
%% A "frame" is a language constructs we can't jump through. For a given top-level module function,
%% effectively this is only `try`'s body and a local fun. To set up alternative forwarding through
%% frames, we need to track which jumps escape the frame (i.e. their target is outside of the frame;
%% no action is needed if the jump targets a label within its own frame).
%%
%% - `jump_escapes`: `[{Label, Arity}]`, the labels outside of the expression's frame that are
%%   targeted by the jumps from within the expression.
%%
%% - `jump_dispatch`: `{[...Landing], [...Forward]}`, for an expression where we must build
%%   a dispatching `case`. The `case` matches on `Landing` jump tuples (tuples built to pass jumps
%%   through a frame) and sets up jumps to local labels. The `Forward` labels are ones that we need
%%   to forward as they are to outside of our current frame. Anything else is a normal return value
%%   of the expression.
%%
%% - `is_frame`: `true` if the expression is a frame.
%%
%% - `fun_type`:
%%   - `{local, NameLabel}` for a local function (letrec's definition that's not a join point),
%%     where `NameLabel` is the label of its name,
%%   - `letrec_goto` for a letrec_goto body,
%%   - `anonymous` for any other fun.
%%
%% - `callee`: for a variable bound to a local function, the label of the function's name.
%%
%% - `callee_jump_escapes`: for a local function's name, the `jump_escapes` of its body,
%%   i.e. the labels it could jump to.
%%
%%   Note that the rewriting logic is built with an assumption that only named local funs have
%%   any escapes - these are loop helpers with `return` or `break`/`continue` to an outer loop.
%%   Also, no code binds a loop to a variable and calls it through the var, and all loops are built
%%   as recursive functions (duh, but has to be stated - erlc only emits `letrec` for a local fun
%%   if it calls itself).
gather(Module, Table) ->
    cerl_trees:mapfold(fun gather_pre/2, fun gather_post/2, #gather{table = Table}, Module),
    ok.

gather_pre(Expr, #gather{table = Table, scope_labels = Scopes} = State0) ->
    State1 =
        case is_frame(Expr, Table) of
            true -> State0#gather{scope_labels = [ordsets:new() | Scopes]};
            false -> State0
        end,

    State =
        case cerl:type(Expr) of
            'let' -> gather_pre_let(Expr, State1);
            letrec -> gather_pre_letrec(Expr, State1);
            'case' -> gather_pre_case(Expr, State1);
            'fun' -> gather_pre_fun(Expr, State1);
            'try' -> gather_pre_try(Expr, State1);
            _ -> State1
        end,

    {Expr, State}.

gather_pre_let(Let, #gather{table = Table} = State) ->
    maybe
        [Var] ?= cerl:let_vars(Let),
        Arg = cerl:let_arg(Let),
        letrec ?= cerl:type(Arg),
        Name = cerl:letrec_body(Arg),
        true ?= cerl:is_c_fname(Name),
        add_fact(Table, callee, Var, label(Name))
    end,
    State.

gather_pre_letrec(Letrec, #gather{table = Table} = State) ->
    IsGoto = lists:member(letrec_goto, cerl:get_ann(Letrec)),
    lists:foreach(
        fun({Name, Fun}) ->
            FunType =
                case IsGoto of
                    true -> letrec_goto;
                    false -> {local, label(Name)}
                end,
            add_fact(Table, fun_type, Fun, FunType)
        end,
        cerl:letrec_defs(Letrec)
    ),
    State.

gather_pre_case(Case, #gather{scope_labels = [Labels0 | Scopes]} = State) ->
    Labels = ordsets:union(join_labels(Case), Labels0),
    State#gather{scope_labels = [Labels | Scopes]}.

gather_pre_fun(Fun, #gather{table = Table} = State) ->
    case get_fact(Table, fun_type, Fun, anonymous) of
        letrec_goto -> ok;
        _ -> mark_frame(Table, cerl:fun_body(Fun))
    end,
    State.

gather_pre_try(Try, #gather{table = Table} = State) ->
    mark_frame(Table, cerl:try_arg(Try)),
    State.

%% A leaf can't contain a jump, so it doesn't need to be a frame.
%% `cerl_trees:label` doesn't give labels to literals, and every `var`'s occurrence shares the same
%% label, so it's simply convenient for us to not mark them as frames.
mark_frame(Table, Expr) ->
    cerl:is_leaf(Expr) orelse add_fact(Table, is_frame, Expr, true).

gather_post(Expr, #gather{table = Table} = State0) ->
    {OwnEscapes, State1} =
        case cerl:type(Expr) of
            call -> gather_post_call(Expr, State0);
            'case' -> gather_post_case(Expr, State0);
            'fun' -> gather_post_fun(Expr, State0);
            'try' -> gather_post_try(Expr, State0);
            apply -> gather_post_apply(Expr, State0);
            _ -> {[], State0}
        end,

    ChildEscapes = [
        get_fact(Table, jump_escapes, Child, [])
     || Group <- cerl:subtrees(Expr),
        Child <- Group,
        %% Skips frame bodies: their escapes leave the frame only through a dispatch.
        not is_frame(Child, Table)
    ],
    add_fact(Table, jump_escapes, Expr, ordsets:union([OwnEscapes | ChildEscapes])),

    State =
        case is_frame(Expr, Table) of
            true -> State1#gather{scope_labels = tl(State1#gather.scope_labels)};
            false -> State1
        end,

    {Expr, State}.

gather_post_call(Call, State) ->
    OwnEscapes =
        maybe
            {Label, Args} ?= jump_marker(Call),
            false ?= in_scope(Label, State),
            [{Label, length(Args)}]
        else
            _ -> []
        end,
    {OwnEscapes, State}.

gather_post_case(Case, #gather{scope_labels = [Labels0 | Scopes]} = State) ->
    Labels = ordsets:subtract(Labels0, join_labels(Case)),
    {[], State#gather{scope_labels = [Labels | Scopes]}}.

gather_post_fun(Fun, #gather{table = Table} = State) ->
    case get_fact(Table, fun_type, Fun, anonymous) of
        {local, NameLabel} ->
            Escapes = get_fact(Table, jump_escapes, cerl:fun_body(Fun), []),
            add_fact(Table, callee_jump_escapes, NameLabel, Escapes);
        _ ->
            ok
    end,
    {[], State}.

gather_post_try(Try, #gather{table = Table} = State) ->
    case get_fact(Table, jump_escapes, cerl:try_arg(Try), []) of
        [] ->
            {[], State};
        FrameEscapes ->
            OwnEscapes = add_jump_dispatch(Try, FrameEscapes, State),
            {OwnEscapes, State}
    end.

gather_post_apply(Apply, #gather{table = Table} = State) ->
    Op = cerl:apply_op(Apply),
    CalleeLabel = get_fact(Table, callee, Op, label(Op)),
    case get_fact(Table, callee_jump_escapes, CalleeLabel, []) of
        [] ->
            {[], State};
        CalleeEscapes ->
            OwnEscapes = add_jump_dispatch(Apply, CalleeEscapes, State),
            {OwnEscapes, State}
    end.

%% Adds the `jump_dispatch` fact for the labels that escape a frame through `Node`: the ones that
%% land in the current frame, and the ones that it forwards further. Returns the forwarded labels.
add_jump_dispatch(Node, Escapes, #gather{table = Table} = State) ->
    {Landing, Forward} = lists:partition(fun({Label, _}) -> in_scope(Label, State) end, Escapes),
    add_fact(Table, jump_dispatch, Node, {Landing, Forward}),
    Forward.

in_scope(Label, #gather{scope_labels = [Labels | _]}) ->
    ordsets:is_element(Label, Labels).

%% Rewrites the '$patina_marker' control-flow markers into jumps.
%%
%% A jump is an apply of a `letrec_goto` join point. This transform is meant to run before
%% any optimization, because the inlining opt can inline a lambda with `return` into parent body.
%%
%% - `jump(Label, Args...)` jumps to `Label/N`, a join point created from a `Join`'s `case`.
%%
%% Labels are unique within a function and its closures. A jump can't leave its frame
%% (a function, or a `try` protected expression). So the same jump has a different effect in each
%% scope:
%%
%% - If the label is defined in the same frame as the jump, it simply jumps there.
%%
%% - If the marker is inside a local function (e.g. an outlined `after` block, or a loop helper),
%%   or inside `try` protected expression, and its label is defined outside, it can't jump out.
%%   It jumps to the frame's `'$patina_forward'/1` join point with a tuple `{Label, Args...}`,
%%   which returns it from the frame.
%%
%% - At call sites of local functions, and after `try`s, we have to have a `case` that peels out
%%   these tuples, and jumps again.
rewrite(Expr, Table) ->
    Rewritten0 =
        case cerl:type(Expr) of
            call -> rewrite_call(Expr, Table);
            seq -> rewrite_seq(Expr);
            'case' -> rewrite_case(Expr);
            apply -> rewrite_apply(Expr, Table);
            'try' -> rewrite_try(Expr, Table);
            _ -> Expr
        end,

    Rewritten = drop_cerl_labels(Rewritten0),

    case is_frame(Expr, Table) andalso has_jump_escapes(Expr, Table) of
        true ->
            Var = cerl:c_var('@patina_jump'),
            join_point('$patina_forward', [Var], Var, Rewritten);
        false ->
            Rewritten
    end.

drop_cerl_labels(Expr) ->
    Anns = lists:filter(
        fun
            ({label, _}) -> false;
            (_) -> true
        end,
        cerl:get_ann(Expr)
    ),
    cerl:set_ann(Expr, Anns).

rewrite_call(Expr, Table) ->
    case jump_marker(Expr) of
        {Label, Args} ->
            case has_jump_escapes(Expr, Table) of
                true ->
                    JumpTuple = cerl:c_tuple([cerl:c_atom(Label) | Args]),
                    cerl:c_apply(cerl:c_fname('$patina_forward', 1), [JumpTuple]);
                false ->
                    cerl:c_apply(cerl:c_fname(Label, length(Args)), Args)
            end;
        false ->
            Expr
    end.

rewrite_apply(Apply, Table) ->
    case get_fact(Table, jump_dispatch, Apply, undefined) of
        undefined -> Apply;
        {Landing, Forward} -> dispatch_on_jump_tuple(Apply, Landing, Forward)
    end.

rewrite_try(Try, Table) ->
    case get_fact(Table, jump_dispatch, Try, undefined) of
        undefined -> Try;
        {Landing, Forward} -> dispatch_on_jump_tuple(Try, Landing, Forward)
    end.

rewrite_seq(Expr) ->
    case is_jump(cerl:seq_arg(Expr)) of
        %% `do Jump Rest`: `Rest` cannot run.
        true -> cerl:seq_arg(Expr);
        false -> Expr
    end.

%% Rewrites a Join's `case` into its body wrapped with a letrec.
rewrite_case(Case) ->
    maybe
        %% erlc may add a fail clause after the body clause.
        [JoinClause, BodyClause | _] ?= cerl:case_clauses(Case),
        {Label, Vars} ?= label_clause(JoinClause),
        join_point(Label, Vars, cerl:clause_body(JoinClause), cerl:clause_body(BodyClause))
    else
        _ -> Case
    end.

%% The special `{'$patina_label_N', ..Args}` clause of a Join's `case`.
label_clause(Clause) ->
    maybe
        [Pat] ?= cerl:clause_pats(Clause),
        true ?= cerl:is_c_tuple(Pat),
        [Label | Vars] ?= cerl:tuple_es(Pat),
        true ?= cerl:is_c_atom(Label),
        true ?= is_label(cerl:atom_val(Label)),

        {cerl:atom_val(Label), Vars}
    else
        _ -> false
    end.

is_jump(Expr) ->
    cerl:is_c_apply(Expr) andalso cerl:is_c_fname(cerl:apply_op(Expr)) andalso
        begin
            Name = cerl:fname_id(cerl:apply_op(Expr)),
            Name =:= '$patina_forward' orelse is_label(Name)
        end.

has_jump_escapes(Expr, Table) ->
    get_fact(Table, jump_escapes, Expr, []) =/= [].

join_point(Name, Vars, Body, WrappedBody) ->
    Arity = length(Vars),
    cerl:ann_c_letrec(
        [letrec_goto, no_inline], [{cerl:c_fname(Name, Arity), cerl:c_fun(Vars, Body)}], WrappedBody
    ).

dispatch_on_jump_tuple(Expr, Landing, Forward) ->
    LandingClauses = lists:map(
        fun({Label, Arity}) ->
            Vars = arg_vars(Arity),
            Pattern = cerl:c_tuple([cerl:c_atom(Label) | Vars]),
            Body = cerl:c_apply(cerl:c_fname(Label, Arity), Vars),
            cerl:c_clause([Pattern], Body)
        end,
        Landing
    ),

    ForwardClauses = lists:map(
        fun({Label, Arity}) ->
            Jump = cerl:c_var('@patina_jump'),
            Pattern = cerl:c_alias(Jump, cerl:c_tuple([cerl:c_atom(Label) | arg_vars(Arity)])),
            Body = cerl:c_apply(cerl:c_fname('$patina_forward', 1), [Jump]),
            cerl:c_clause([Pattern], Body)
        end,
        Forward
    ),

    Other = cerl:c_var('@patina_other'),
    Clauses = LandingClauses ++ ForwardClauses ++ [cerl:c_clause([Other], Other)],

    Value = cerl:c_var('@patina_value'),
    cerl:c_let([Value], Expr, cerl:c_case(Value, Clauses)).

jump_marker(Expr) ->
    maybe
        true ?= cerl:is_c_call(Expr),
        true ?= cerl:is_c_atom(cerl:call_module(Expr)),
        '$patina_marker' ?= cerl:atom_val(cerl:call_module(Expr)),
        true ?= cerl:is_c_atom(cerl:call_name(Expr)),
        jump ?= cerl:atom_val(cerl:call_name(Expr)),

        [Label | Args] = cerl:call_args(Expr),
        {cerl:atom_val(Label), Args}
    else
        _ -> false
    end.

join_labels(Case) ->
    maybe
        [Clause | _] ?= cerl:case_clauses(Case),
        {Label, _} ?= label_clause(Clause),
        [Label]
    else
        _ -> []
    end.

is_label(Name) ->
    lists:prefix("$patina_label_", atom_to_list(Name)).

is_frame(Expr, Table) ->
    get_fact(Table, is_frame, Expr, false).

add_fact(_Table, _Key, _Node, []) ->
    true;
add_fact(Table, Key, Node, Value) ->
    ets:insert(Table, {{Key, label(Node)}, Value}).

get_fact(Table, Key, Node, Default) ->
    case ets:lookup(Table, {Key, label(Node)}) of
        [{_, Value}] -> Value;
        [] -> Default
    end.

label(Label) when is_integer(Label) ->
    Label;
label(Node) ->
    case cerl:get_ann(Node) of
        [{label, Label} | _] -> Label;
        _ -> undefined
    end.

arg_vars(N) ->
    [cerl:c_var(list_to_atom("@patina_arg_" ++ integer_to_list(I))) || I <- lists:seq(1, N)].

-spec tmp_dir() -> file:filename() | no_return().
tmp_dir() ->
    Candidates = [
        {env, "TMPDIR"}, {env, "TEMP"}, {env, "TMP"}, {dir, "/tmp"}, {dir, "/var/tmp"}, cwd
    ],
    case find_writable_tmp_dir(Candidates) of
        false -> error({no_writable_tmp_dir, Candidates});
        TmpDir -> TmpDir
    end.

find_writable_tmp_dir([Candidate | Candidates]) ->
    maybe
        [_ | _] = Dir ?= resolve_tmp_dir(Candidate),
        true ?= is_writable_dir(Dir),
        Dir
    else
        _ -> find_writable_tmp_dir(Candidates)
    end;
find_writable_tmp_dir([]) ->
    false.

resolve_tmp_dir({env, Var}) ->
    os:getenv(Var);
resolve_tmp_dir({dir, Dir}) ->
    Dir;
resolve_tmp_dir(cwd) ->
    case file:get_cwd() of
        {ok, Dir} -> Dir;
        {error, _} -> false
    end.

is_writable_dir(Dir) ->
    case file:read_file_info(Dir) of
        {ok, #file_info{type = directory, access = Access}} ->
            Access =:= read_write orelse Access =:= write;
        {error, _} ->
            false
    end.
