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

handle_call(_Request, _From, Sock) ->
    {reply, ok, Sock}.

handle_cast(_Msg, Sock) ->
    {noreply, Sock}.

handle_continue(accept, Sock) ->
    Deadline = erlang:monotonic_time(millisecond) + ?ACCEPT_TIMEOUT,
    case accept_client(Sock, Deadline) of
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
    {Pid, Ref} = spawn_opt(fun() ->
        Parent = receive {reply_to, Parent0} -> Parent0 end,

        Result = try
            {ok, apply(Module, Function, Args)}
        catch
            Class:Reason:Stack ->
                FilteredStack = lists:takewhile(fun
                    ({?MODULE, _, _, _}) -> false;
                    (_) -> true
                end, Stack),
                {error, {Class, Reason, FilteredStack}}
        end,
        Parent ! {Parent, Result}
    end, [{monitor, [{alias, reply_demonitor}]}]),

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
    CompileOpts = [binary, return, nowarn_opportunistic, nowarn_underscore_match],
    case compile:noenv_forms(Forms, [to_asm, no_postopt | CompileOpts]) of
        {ok, _Mod, Asm0, Warnings0} ->
            {ok, Asm1} = beam_a:module(Asm0, []),
            Asm2 = rewrite_asm(Asm1),
            case compile:noenv_forms(Asm2, [from_asm | CompileOpts]) of
                {ok, Mod, Asm, Warnings1} ->
                    {ok, Mod, Asm, lists:uniq(Warnings0 ++ Warnings1)};
                {error, Errors, Warnings1} ->
                    {error, Errors, lists:uniq(Warnings0 ++ Warnings1)}
            end;
        {error, Errors, Warnings} ->
            {error, Errors, Warnings}
    end.

rewrite_asm({Module, Exports0, Attributes, Meta, Functions0, LabelCount}) ->
    Exports = drop_loop_helper_exports(Exports0),
    Functions = lists:map(fun rewrite_function/1, Functions0),
    {Module, Exports, Attributes, Meta, Functions, LabelCount}.

%% Loop helpers are exported only as an optimizer fence to prevent them from being specialized.
drop_loop_helper_exports(Exports) ->
    lists:filter(fun({Name, _Arity}) ->
        string:find(atom_to_list(Name), "$patina_loop_") =:= nomatch
    end, Exports).

rewrite_function({function, Name, Arity, Entry, Instrs} = Function) ->
    case lists:any(fun is_control_marker/1, Instrs) of
        true -> {function, Name, Arity, Entry, rewrite_control_markers(Instrs, Entry)};
        false -> Function
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

%% Rewrites '$patina_marker' control-flow markers to BEAM control flow.
%% The instructions have to be unfolded with `beam_a` first.
%% This function traces state needed for return cleanup (allocation size, try/catch scopes)
%% and ensures that the return markers are replaced with proper cleanup instructions.
rewrite_control_markers(Instrs, FunctionEntry) ->
    map_instrs(
        fun(Instr0, {Frame0, Scopes0, EntryStates0}) ->
            {Frame1, Scopes1} =
                case Instr0 of
                    {label, NewLabel} ->
                        {InheritedFrame, InheritedScopes} = maps:get(
                            NewLabel, EntryStates0, {no_frame, []}
                        ),
                        {InheritedFrame, InheritedScopes};
                    _ ->
                        {Frame0, Scopes0}
                end,

            Frame =
                case stack_allocations(Instr0) of
                    nil -> Frame1;
                    NewFrame -> NewFrame
                end,

            Scopes2 =
                case is_scope_end(Instr0, Scopes1) of
                    true -> tl(Scopes1);
                    false -> Scopes1
                end,
            Scopes =
                case is_scope(Instr0) of
                    true -> [Instr0 | Scopes2];
                    false -> Scopes2
                end,

            EntryStates = lists:foldl(
                fun(NeighborLabel, EntryStatesAcc) ->
                    maps:put(NeighborLabel, {Frame, Scopes}, EntryStatesAcc)
                end,
                EntryStates0,
                instr_labels(Instr0)
            ),

            Instr =
                case control_marker(Instr0) of
                    {Marker, Arity} ->
                        RecursiveCall =
                            case Marker of
                                continue -> [{call, Arity, {f, FunctionEntry}}];
                                _ -> []
                            end,

                        Deallocate =
                            case Frame of
                                no_frame -> [];
                                _ -> [{deallocate, Frame}]
                            end,

                        lists:map(fun scope_end/1, Scopes) ++ RecursiveCall ++ Deallocate ++ [return];

                    nil ->
                        Instr0
                end,

            {Instr, {Frame, Scopes, EntryStates}}
        end,
        {no_frame, [], #{}},
        Instrs
    ).

is_control_marker(Instr) -> control_marker(Instr) =/= nil.

control_marker({call_ext, _, {extfunc, '$patina_marker', Marker, Arity}}) -> {Marker, Arity};
control_marker(_) -> nil.

stack_allocations({allocate, Need, _Live}) -> Need;
stack_allocations({allocate_zero, Need, _Live}) -> Need;
stack_allocations({allocate_heap, Need, _HeapNeed, _Live}) -> Need;
stack_allocations({allocate_heap_zero, Need, _HeapNeed, _Live}) -> Need;
stack_allocations({deallocate, _N}) -> no_frame;
stack_allocations(_) -> nil.

is_scope_end({catch_end, Y}, [{'catch', Y, _F} | _]) -> true;
is_scope_end({try_end, Y}, [{'try', Y, _H} | _]) -> true;
is_scope_end({try_case, Y}, [{'try', Y, _H} | _]) -> true;
is_scope_end(_, _) -> false.

is_scope(Instr) -> scope_end(Instr) =/= [].

scope_end({'catch', Y, _F}) -> [{catch_end, Y}];
scope_end({'try', Y, _H}) -> [{try_end, Y}];
scope_end(_) -> [].

%% Orders instruction groups in BFS (topsort) order.
%% We're ordering them using BFS, because technically instructions can jump backwards,
%% so it's not enough to just process them in order of occurence.
%% After ordering, the groups are flattened into a single list of instructions
%% in the order of occurence.
map_instrs(Mapper, UserAcc, Instrs) ->
    Seen = initial_labels(Instrs),
    Queue = queue:from_list(sets:to_list(Seen)),
    Groups0 = group_by_label(Instrs),
    Groups = map_instrs(Queue, Seen, Groups0, Mapper, UserAcc),

    lists:flatten(
        lists:map(
            fun
                ({label, Label}) -> maps:get(Label, Groups);
                (_Instr) -> []
            end,
            Instrs
        )
    ).

map_instrs(Queue0, Seen0, Groups0, Mapper, UserAcc0) ->
    case queue:out(Queue0) of
        {{value, Label}, Queue1} ->
            Instrs0 = maps:get(Label, Groups0),
            Neighbors = lists:foldl(fun ulbl/2, sets:new(), Instrs0),
            UnseenNeighbors = sets:subtract(Neighbors, Seen0),
            Seen = sets:union(Seen0, UnseenNeighbors),
            Queue = sets:fold(fun queue:in/2, Queue1, UnseenNeighbors),

            {Instrs, UserAcc} = lists:mapfoldl(Mapper, UserAcc0, Instrs0),
            Groups = Groups0#{Label := Instrs},

            map_instrs(Queue, Seen, Groups, Mapper, UserAcc);
        {empty, _} ->
            Groups0
    end.

%% Traverses the instruction list, grouping them by label.
%% Returns a map of labels to instruction lists. If an instruction falls through, the list
%% will also contain an explicit jump to the fallthrough label.
group_by_label(Instrs) ->
    group_by_label(Instrs, 0, [], #{}).

group_by_label([] = _Instr, _Label, [] = _InstrAcc, Groups) ->
    Groups;
group_by_label([] = _Instr, Label, InstrAcc, Groups) ->
    Groups#{Label => lists:reverse(InstrAcc)};
group_by_label([{label, NewLabel} = Instr | Rest], Label, InstrAcc, Groups0) ->
    Groups = Groups0#{Label => lists:reverse(InstrAcc)},
    group_by_label(Rest, NewLabel, [Instr], Groups);
group_by_label([Instr | [{label, NextLabel} | _] = Rest], Label, InstrAcc, Groups) ->
    Jump =
        case extended_is_unreachable_after(Instr) of
            true -> [];
            false -> [{jump, {f, NextLabel}}]
        end,
    group_by_label(Rest, Label, Jump ++ [Instr | InstrAcc], Groups);
group_by_label([Instr | Rest], Label, InstrAcc, Groups) ->
    group_by_label(Rest, Label, [Instr | InstrAcc], Groups).

%% beam_jump's `is_unreachable_after` function doesn't care about exit bifs,
%% but we do because we have to know whether fallthrough is possible or not.
extended_is_unreachable_after(Instr) ->
    case Instr of
        {call_ext, _, {_, M, F, A}} -> erl_bifs:is_exit_bif(M, F, A);
        {bif, raise, _, _, _} -> true;
        _ -> is_unreachable_after(Instr)
    end.

%% -----------------------------------------------------------------------------
%% Functions copied verbatim from Erlang/OTP's `beam_jump` module.
%% Keep them synced with BEAM, and don't modify them otherwise.
%% -----------------------------------------------------------------------------

initial_labels(Is) ->
    initial_labels(Is, []).

initial_labels([{line, _} | Is], Acc) ->
    initial_labels(Is, Acc);
initial_labels([{label, Lbl} | Is], Acc) ->
    initial_labels(Is, [Lbl | Acc]);
initial_labels([{func_info, _, _, _}, {label, Lbl} | _], Acc) ->
    sets:from_list([Lbl | Acc]).

is_unreachable_after({func_info, _M, _F, _A}) -> true;
is_unreachable_after(return) -> true;
is_unreachable_after({jump, _Lbl}) -> true;
is_unreachable_after({select, _What, _R, _Lbl, _Cases}) -> true;
is_unreachable_after({loop_rec_end, _}) -> true;
is_unreachable_after({wait, _}) -> true;
is_unreachable_after(I) -> is_exit_instruction(I).

is_exit_instruction(if_end) -> true;
is_exit_instruction({case_end, _}) -> true;
is_exit_instruction({try_case_end, _}) -> true;
is_exit_instruction({badmatch, _}) -> true;
is_exit_instruction({badrecord, _}) -> true;
is_exit_instruction(_) -> false.

ulbl(I, Used) ->
    case instr_labels(I) of
        [] ->
            Used;
        [Lbl] ->
            sets:add_element(Lbl, Used);
        [_ | _] = L ->
            ulbl_list(L, Used)
    end.

ulbl_list([L | Ls], Used) ->
    ulbl_list(Ls, sets:add_element(L, Used));
ulbl_list([], Used) ->
    Used.

instr_labels({test, _, Fail, _}) ->
    do_instr_labels(Fail);
instr_labels({test, _, Fail, _, _, _}) ->
    do_instr_labels(Fail);
instr_labels({select, _, _, Fail, Vls}) ->
    do_instr_labels_list(Vls, do_instr_labels(Fail));
instr_labels({'try', _, Lbl}) ->
    do_instr_labels(Lbl);
instr_labels({'catch', _, Lbl}) ->
    do_instr_labels(Lbl);
instr_labels({jump, Lbl}) ->
    do_instr_labels(Lbl);
instr_labels({loop_rec, Lbl, _}) ->
    do_instr_labels(Lbl);
instr_labels({loop_rec_end, Lbl}) ->
    do_instr_labels(Lbl);
instr_labels({wait, Lbl}) ->
    do_instr_labels(Lbl);
instr_labels({wait_timeout, Lbl, _To}) ->
    do_instr_labels(Lbl);
instr_labels({bif, _Name, Lbl, _As, _R}) ->
    do_instr_labels(Lbl);
instr_labels({gc_bif, _Name, Lbl, _Live, _As, _R}) ->
    do_instr_labels(Lbl);
instr_labels({bs_create_bin, Lbl, _, _, _, _, _}) ->
    do_instr_labels(Lbl);
instr_labels({put_map, Lbl, _Op, _Src, _Dst, _Live, _List}) ->
    do_instr_labels(Lbl);
instr_labels({get_map_elements, Lbl, _Src, _List}) ->
    do_instr_labels(Lbl);
instr_labels({get_record_elements, Lbl, _Src, _List}) ->
    do_instr_labels(Lbl);
instr_labels({bs_start_match4, Fail, _, _, _}) ->
    case Fail of
        {f, L} -> [L];
        {atom, _} -> []
    end;
instr_labels({bs_match, {f, Fail}, _Ctx, _List}) ->
    [Fail];
instr_labels(_) ->
    [].

do_instr_labels({f, 0}) -> [];
do_instr_labels({f, F}) -> [F].

do_instr_labels_list([{f, F} | T], Acc) ->
    do_instr_labels_list(T, [F | Acc]);
do_instr_labels_list([_ | T], Acc) ->
    do_instr_labels_list(T, Acc);
do_instr_labels_list([], Acc) ->
    Acc.
