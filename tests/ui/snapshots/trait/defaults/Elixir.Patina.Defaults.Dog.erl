-file("tests/ui/src/trait/defaults.pt", 5).

-module('Elixir.Patina.Defaults.Dog').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Animal::name'/2,
         'defaults::Animal::noise'/2,
         'defaults::Animal::calls_required'/2,
         'defaults::Animal::$patina_witness'/0]).

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

'defaults::Animal::name'(_witness_Animal@0_0,
                         _self@1_0) ->
    _self@1_0.

'defaults::Animal::noise'(_witness_Animal@0_0,
                          _self@1_0) ->
    <<"woof"/utf8>>.

'defaults::Animal::calls_required'(_witness_Animal@0_0,
                                   _self@1_0) ->
    _element@2_0 = _self@1_0,
    {_element@2_0, <<"barks"/utf8>>}.

'defaults::Animal::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Dog':'defaults::Animal::name'/2,
      fun 'Elixir.Patina.Defaults.Dog':'defaults::Animal::noise'/2,
      fun 'Elixir.Patina.Defaults.Dog':'defaults::Animal::calls_required'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::calls_default'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::with_bounded_param'/4,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::returns_self'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::whisper'/2},
     {},
     {}}.
