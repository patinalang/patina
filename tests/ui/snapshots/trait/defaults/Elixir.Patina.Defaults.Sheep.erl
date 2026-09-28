-file("tests/ui/src/trait/defaults.pt", 1).

-module('Elixir.Patina.Defaults.Sheep').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Animal::name'/2,
         'Defaults::Animal::noise'/2,
         'Defaults::Animal::calls_required'/2,
         'Defaults::Animal::calls_default'/2,
         'Defaults::Animal::with_bounded_param'/4,
         'Defaults::Animal::returns_self'/2,
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

'Defaults::Animal::name'(_Self@0_0, _self@1_0) ->
    _self@1_0.

'Defaults::Animal::noise'(_Self@0_0, _self@1_0) ->
    <<"baaaaah"/utf8>>.

'Defaults::Animal::calls_required'(_arg@0_0,
                                   _arg@1_0) ->
    'Elixir.Patina.Defaults':'Defaults::Animal::calls_required'(_arg@0_0,
                                                                _arg@1_0).

'Defaults::Animal::calls_default'(_arg@0_0, _arg@1_0) ->
    'Elixir.Patina.Defaults':'Defaults::Animal::calls_default'(_arg@0_0,
                                                               _arg@1_0).

'Defaults::Animal::with_bounded_param'(_arg@0_0,
                                       _arg@1_0, _arg@2_0, _arg@3_0) ->
    'Elixir.Patina.Defaults':'Defaults::Animal::with_bounded_param'(_arg@0_0,
                                                                    _arg@1_0,
                                                                    _arg@2_0,
                                                                    _arg@3_0).

'Defaults::Animal::returns_self'(_arg@0_0, _arg@1_0) ->
    'Elixir.Patina.Defaults':'Defaults::Animal::returns_self'(_arg@0_0,
                                                              _arg@1_0).

'Defaults::Animal::whisper'(_arg@0_0, _arg@1_0) ->
    'Elixir.Patina.Defaults':'Defaults::Animal::whisper'(_arg@0_0,
                                                         _arg@1_0).

calls_stubs_locally(_self@0_0) ->
    {'Defaults::Animal::calls_required'({'Elixir.Patina.Defaults.Sheep',
                                         {}},
                                        _self@0_0),
     'Defaults::Animal::whisper'({'Elixir.Patina.Defaults.Sheep',
                                  {}},
                                 _self@0_0)}.
