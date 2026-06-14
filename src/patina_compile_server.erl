-module(patina_compile_server).

-export([start_link/1]).

-behaviour(gen_server).
-export([init/1, handle_call/3, handle_cast/2,  handle_continue/2]).

start_link(SockPath) ->
    gen_server:start_link(?MODULE, [SockPath], []).

init([SockPath]) ->
    {ok, Socket} = socket:open(local, stream, default, #{}),
    socket:bind(Socket, #{family => local, path => SockPath}),
    socket:listen(Socket),
    {ok, Socket, {continue, accept}}.

handle_call(_Request, _From, Sock) ->
    {reply, ok, Sock}.

handle_cast(_Msg, Sock) ->
    {noreply, Sock}.

handle_continue(accept, Sock) ->
    {ok, ClientSock} = socket:accept(Sock),
    socket_loop(ClientSock),
    socket:close(ClientSock),
    {noreply, Sock, {continue, accept}}.

socket_loop(Sock) ->
    case read_msg(Sock) of
        {ok, [Op, Eaf]} ->
            handle_request(Sock, Op, Eaf),
            socket_loop(Sock);
        closed ->
            ok
    end.

handle_request(Sock, Op, Eaf) ->
    Forms = binary_to_term(Eaf),
    try
        case Op of
            ~"compile" ->
                case compile:noenv_forms(Forms, [binary, return]) of
                    {ok, _Mod, Beam, Warnings} ->
                        write_msg(Sock, [~"ok", Beam, Warnings]);

                    {error, Errors, Warnings} ->
                        write_msg(Sock, [~"error", Errors, Warnings])
                end;

            ~"abstr" ->
                Rendered = iolist_to_binary(io_lib:format("~tp", [Forms])),
                write_msg(Sock, [~"ok", Rendered]);

            ~"erl" ->
                Rendered = iolist_to_binary([erl_pp:form(Fm) || Fm <- Forms]),
                write_msg(Sock, [~"ok", Rendered])
        end
    catch Class:Reason ->
        write_msg(Sock, [~"raise", Class, Reason])
    end.

read_msg(Sock) ->
    case socket:recv(Sock, 1) of
        {ok, <<Len:8/unsigned-integer>>} ->
            {ok, lists:map(fun(_) -> read_bin(Sock) end, lists:seq(1, Len))};
        {error, closed} ->
            closed
    end.

read_bin(Sock) ->
    {ok, <<Len:32/unsigned-integer>>} = socket:recv(Sock, 4),
    {ok, Data} = socket:recv(Sock, Len),
    Data.

write_msg(Sock, [_|_] = Msg) ->
    Len = length(Msg),
    socket:send(Sock, <<Len:8/unsigned-integer>>),
    lists:foreach(fun(Data) -> write(Sock, Data) end, Msg).

write(Sock, <<Data/binary>>) ->
    Len = byte_size(Data),
    socket:send(Sock, <<Len:32/unsigned-integer>>),
    socket:send(Sock, Data);
write(Sock, Data) ->
    Bin = iolist_to_binary(io_lib:format("~tp", [Data])),
    write(Sock, Bin).
