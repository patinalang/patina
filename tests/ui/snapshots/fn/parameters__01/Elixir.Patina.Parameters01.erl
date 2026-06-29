-file("tests/ui/src/fn/parameters__01.pt", 4).
-module('Elixir.Patina.Parameters01').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,main/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
call_me_ref(_var_0) ->
    _var_0(1).
call_me(_var_0) ->
    _var_0(2).
function(_var_0) ->
    _var_0 + 1.
main() ->
    _var_1 =
        fun(_var_0) ->
               _var_0 + 42
        end,
    call_me(fun 'Elixir.Patina.Parameters01':function/1),
    call_me(_var_1),
    call_me_ref(fun 'Elixir.Patina.Parameters01':function/1).
