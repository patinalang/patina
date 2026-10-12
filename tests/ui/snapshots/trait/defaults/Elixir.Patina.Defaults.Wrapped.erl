-file("tests/ui/src/trait/defaults.pt", 284).

-module('Elixir.Patina.Defaults.Wrapped').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Container::$patina_type::Item'/1,
         'Defaults::Container::get'/2,
         'Defaults::Container::$patina_witness'/1]).

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
                                              {_,
                                               {},
                                               {_witness_where_clause@1_0}}) ->
    {}.

'Defaults::Container::get'(_witness_self@0_0 = {_,
                                                {},
                                                {_witness_where_clause@1_0}},
                           _self@2_0) ->
    _self@2_0.

'Defaults::Container::$patina_witness'(_witness_bounds@1_0 =
                                           {_witness_bound@0_0}) ->
    {{fun 'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::$patina_type::Item'/1,
      fun 'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::get'/2,
      fun 'Elixir.Patina.Defaults':'Defaults::Container::needs_item_witness'/3},
     {},
     _witness_bounds@1_0}.
