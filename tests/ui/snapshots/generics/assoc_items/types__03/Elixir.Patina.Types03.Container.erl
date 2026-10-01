-file("tests/ui/src/generics/assoc_items/types__03.pt",
      4).

-module('Elixir.Patina.Types03.Container').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Container']).

-export(['__info__'/1,
         '$patina_constructor__Container'/2,
         'Types03::Contains::type::A'/1,
         'Types03::Contains::type::B'/1,
         'Types03::Contains::contains'/4,
         'Types03::Contains::first'/2,
         'Types03::Contains::last'/2]).

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

-record #'Container'{'0', '1'}.

'$patina_constructor__Container'(_0@0_0, _1@1_0) ->
    #'Container'{'0' = _0@0_0, '1' = _1@1_0}.

'Types03::Contains::type::A'(_witness@0_0 =
                                 {'impl Types03::Contains for Types03::Container',
                                  _}) ->
    {}.

'Types03::Contains::type::B'(_witness@0_0 =
                                 {'impl Types03::Contains for Types03::Container',
                                  _}) ->
    {}.

'Types03::Contains::contains'(_witness@0_0 =
                                  {'impl Types03::Contains for Types03::Container',
                                   _},
                              _self@1_0, _number_1@2_0, _number_2@3_0) ->
    _self@1_0#'Container'.'0' =:= _number_1@2_0 andalso
        _self@1_0#'Container'.'1' =:= _number_2@3_0.

'Types03::Contains::first'(_witness@0_0 =
                               {'impl Types03::Contains for Types03::Container',
                                _},
                           _self@1_0) ->
    _self@1_0#'Container'.'0'.

'Types03::Contains::last'(_witness@0_0 =
                              {'impl Types03::Contains for Types03::Container',
                               _},
                          _self@1_0) ->
    _self@1_0#'Container'.'1'.
