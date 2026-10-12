-file("tests/ui/src/trait/defaults.pt", 268).

-module('Elixir.Patina.Defaults.Boxed').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Container::$patina_type::Item'/1,
         'Defaults::Container::get'/2,
         'Defaults::Container::$patina_witness'/0,
         'Defaults::Describe::describe'/2,
         'Defaults::Describe::$patina_witness'/0,
         'Defaults::Maker::$patina_witness'/0]).

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

'Defaults::Container::$patina_type::Item'(_witness_self@0_0 =
                                              {_, {}, {}}) ->
    {}.

'Defaults::Container::get'(_witness_self@0_0 = {_,
                                                {},
                                                {}},
                           _self@1_0) ->
    _self@1_0.

'Defaults::Container::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Boxed':'Defaults::Container::$patina_type::Item'/1,
      fun 'Elixir.Patina.Defaults.Boxed':'Defaults::Container::get'/2,
      fun 'Elixir.Patina.Defaults':'Defaults::Container::needs_item_witness'/3},
     {},
     {}}.

'Defaults::Describe::describe'(_witness_self@0_0 = {_,
                                                    {},
                                                    {}},
                               _self@1_0) ->
    <<"boxed"/utf8>>.

'Defaults::Describe::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults.Boxed':'Defaults::Describe::describe'/2},
     {},
     {}}.

'Defaults::Maker::$patina_witness'() ->
    {{fun 'Elixir.Patina.Defaults':'Defaults::Maker::make'/2},
     {},
     {}}.
