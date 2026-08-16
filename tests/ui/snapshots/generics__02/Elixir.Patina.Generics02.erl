-file("tests/ui/src/generics__02.pt", 5).
-module('Elixir.Patina.Generics02').
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
    __s@0_0 =
        ('Elixir.Patina.Generics02.Single':'__struct__'())#{'0' :=
                                                                'Elixir.Patina.Generics02.A'},
    __char@1_0 =
        ('Elixir.Patina.Generics02.SingleGen':'__struct__'())#{'0' := $a},
    __t@2_0 =
        ('Elixir.Patina.Generics02.SingleGen':'__struct__'())#{'0' :=
                                                                   'Elixir.Patina.Generics02.A'},
    __int@3_0 =
        ('Elixir.Patina.Generics02.SingleGen':'__struct__'())#{'0' := 6},
    __char@4_0 =
        ('Elixir.Patina.Generics02.SingleGen':'__struct__'())#{'0' := $a}.
