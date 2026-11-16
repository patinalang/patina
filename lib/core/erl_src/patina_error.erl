-module(patina_error).

-compile([nowarn_undefined_behaviour]).

-behaviour(erl_error).
-export([format_error/2]).

-behaviour('Elixir.Exception').
-export([exception/1, message/1]).

-behaviour('Elixir.Module').
-export(['__struct__'/0, '__struct__'/1, '__info__'/1]).

format_error(Error, _Stacktrace) ->
    #{reason => message(Error)}.

exception(Message) ->
    '__struct__'([{message, Message}]).

message(#{message := Message}) ->
    Message.

'__info__'(attributes) ->
    [];
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    [
        #{field => '__exception__', default => true},
        #{field => message, required => true, default => nil}
    ];
'__info__'(What) ->
    module_info(What).

'__struct__'() ->
    #{'__struct__' => ?MODULE, '__exception__' => true, message => nil}.

'__struct__'(KV) ->
    lists:foldl(fun({K, V}, S) -> S#{K := V} end, '__struct__'(), KV).
