-file("tests/ui/src/fn/inline.pt", 93).

-module('Elixir.Patina.Inline.Square').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'inline::Shape::area'/2,
         'inline::Shape::$patina_witness'/0,
         side/1]).

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

'inline::Shape::area'(_witness_Shape@0_0, _self@1_0) ->
    _self@1_0 * _self@1_0.

'inline::Shape::$patina_witness'() ->
    {{fun 'Elixir.Patina.Inline.Square':'inline::Shape::area'/2,
      fun 'Elixir.Patina.Inline':'inline::Shape::double_area'/2},
     {},
     {}}.

side(_self@0_0) -> _self@0_0.
