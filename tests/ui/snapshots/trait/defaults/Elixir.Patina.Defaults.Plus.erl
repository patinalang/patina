-file("tests/ui/src/trait/defaults.pt", 381).

-module('Elixir.Patina.Defaults.Plus').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Numbered::$patina_const::C'/1,
         'defaults::Numbered::$patina_witness'/1]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) -> nil;
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'defaults::Numbered::$patina_const::C'(_witness_Numbered@0_0) ->
    _where_Numbered@1_0 = erlang:element(3,
                                         _witness_Numbered@0_0),
    _witness_Numbered@2_0 = erlang:element(1,
                                           _where_Numbered@1_0),
    _vtable_Numbered@3_0 = erlang:element(1,
                                          _witness_Numbered@2_0),
    _const_fn_Numbered_C@4_0 = erlang:element(1,
                                              _vtable_Numbered@3_0),
    _const_Numbered_C@5_0 =
        _const_fn_Numbered_C@4_0(_witness_Numbered@2_0),
    _const_Numbered_C@5_0 + 10.

'defaults::Numbered::$patina_witness'(_witnesses_where_Numbered@0_0) ->
    {{fun 'Elixir.Patina.Defaults.Plus':'defaults::Numbered::$patina_const::C'/1,
      fun 'Elixir.Patina.Defaults':'defaults::Numbered::$patina_const::D'/1,
      fun 'Elixir.Patina.Defaults':'defaults::Numbered::consts'/2},
     {},
     _witnesses_where_Numbered@0_0}.
