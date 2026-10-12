-file("tests/ui/src/generics/assoc_items/the_problem"
      "__01.pt",
      4).

-module('Elixir.Patina.TheProblem01.Container').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Container']).

-export(['__info__'/1,
         '$patina_constructor__Container'/2,
         'TheProblem01::Contains<int, int>::contains'/4,
         'TheProblem01::Contains<int, int>::first'/2,
         'TheProblem01::Contains<int, int>::last'/2,
         'TheProblem01::Contains<int, int>::$patina_witness'/0]).

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

'TheProblem01::Contains<int, int>::contains'(_witness@0_0,
                                             _self@1_0, _number_1@2_0,
                                             _number_2@3_0) ->
    _self@1_0#'Container'.'0' =:= _number_1@2_0 andalso
        _self@1_0#'Container'.'1' =:= _number_2@3_0.

'TheProblem01::Contains<int, int>::first'(_witness@0_0,
                                          _self@1_0) ->
    _self@1_0#'Container'.'0'.

'TheProblem01::Contains<int, int>::last'(_witness@0_0,
                                         _self@1_0) ->
    _self@1_0#'Container'.'1'.

'TheProblem01::Contains<int, int>::$patina_witness'() ->
    {{fun 'Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::contains'/4,
      fun 'Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::first'/2,
      fun 'Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::last'/2},
     {},
     {}}.
