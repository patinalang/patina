-file("tests/ui/src/generics/witness/duplicate_preco"
      "nditions.pt",
      19).

-module('Elixir.Patina.DuplicatePreconditions.$impl.Core.Int').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'duplicate_preconditions::Bar::bar'/2,
         'duplicate_preconditions::Bar::$patina_witness'/0,
         'duplicate_preconditions::Left::left'/2,
         'duplicate_preconditions::Left::$patina_witness'/0,
         'duplicate_preconditions::Right::right'/2,
         'duplicate_preconditions::Right::$patina_witness'/0]).

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

'duplicate_preconditions::Bar::bar'(_witness_Bar@0_0,
                                    _self@1_0) ->
    _self@1_0.

'duplicate_preconditions::Bar::$patina_witness'() ->
    {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'duplicate_preconditions::Bar::bar'/2},
     {},
     {}}.

'duplicate_preconditions::Left::left'(_witness_Left@0_0,
                                      _self@1_0) ->
    0.

'duplicate_preconditions::Left::$patina_witness'() ->
    _witness_Bar@0_0 =
        {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'duplicate_preconditions::Bar::bar'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'duplicate_preconditions::Left::left'/2},
     {_witness_Bar@0_0},
     {}}.

'duplicate_preconditions::Right::right'(_witness_Right@0_0,
                                        _self@1_0) ->
    0.

'duplicate_preconditions::Right::$patina_witness'() ->
    _witness_Bar@0_0 =
        {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'duplicate_preconditions::Bar::bar'/2},
         {},
         {}},
    {{fun 'Elixir.Patina.DuplicatePreconditions.$impl.Core.Int':'duplicate_preconditions::Right::right'/2},
     {_witness_Bar@0_0},
     {}}.
