-file("tests/ui/src/trait/defaults.pt", 132).

-module('Elixir.Patina.Defaults.Pile').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Value::value'/2,
         'Defaults::Value::$patina_witness'/1,
         'Defaults::Combine::$patina_witness'/1]).

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

'Defaults::Value::value'(_witness@0_0, _self@1_0) ->
    begin
        _witness@2_0 = erlang:element(1,
                                      erlang:element(3, _witness@0_0)),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                          _self@1_0)
    end
        + 1.

'Defaults::Value::$patina_witness'(_witness@0_0) ->
    {{fun 'Elixir.Patina.Defaults.Pile':'Defaults::Value::value'/2},
     {},
     _witness@0_0}.

'Defaults::Combine::$patina_witness'(_witness@0_0) ->
    {{fun 'Elixir.Patina.Defaults':'Defaults::Combine::uses_rhs_witness'/4},
     {{{fun 'Elixir.Patina.Defaults.Pile':'Defaults::Value::value'/2},
       {},
       {erlang:element(1, _witness@0_0)}}},
     _witness@0_0}.
