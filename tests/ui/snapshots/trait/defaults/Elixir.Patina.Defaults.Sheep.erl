-file("tests/ui/src/trait/defaults.pt", 1).

-module('Elixir.Patina.Defaults.Sheep').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'defaults::Animal::name'/2,
         'defaults::Animal::noise'/2,
         'defaults::Animal::$patina_witness'/0,
         calls_stubs_locally/1]).

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
    <<"baaaaah"/utf8>>.

'defaults::Animal::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Sheep':'defaults::Animal::name'/2,
      fun 'Elixir.Patina.Defaults.Sheep':'defaults::Animal::noise'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::calls_required'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::calls_default'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::with_bounded_param'/4,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::returns_self'/2,
      fun 'Elixir.Patina.Defaults':'defaults::Animal::whisper'/2},
     {},
     {}}.

calls_stubs_locally(_self@0_0) ->
    _witness_Animal@1_0 =
        'defaults::Animal::$patina_witness'(),
    _element@2_0 =
        'Elixir.Patina.Defaults':'defaults::Animal::calls_required'(_witness_Animal@1_0,
                                                                    _self@0_0),
    _element@3_0 =
        'Elixir.Patina.Defaults':'defaults::Animal::whisper'(_witness_Animal@1_0,
                                                             _self@0_0),
    {_element@2_0, _element@3_0}.
