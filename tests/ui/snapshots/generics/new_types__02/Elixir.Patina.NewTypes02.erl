-file("tests/ui/src/generics/new_types__02.pt", 4).
-module('Elixir.Patina.NewTypes02').
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
main() ->
    _var_0 =
        ('Elixir.Patina.NewTypes02.Years':'__struct__'())#{'0' := 42},
    _var_1 = map_get('0', _var_0),
    #{'0' := _var_2} = _var_0.
