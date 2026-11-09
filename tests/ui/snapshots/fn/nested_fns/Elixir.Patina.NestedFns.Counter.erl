-file("tests/ui/src/fn/nested_fns.pt", 68).

-module('Elixir.Patina.NestedFns.Counter').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         bump/1,
         'nested_fns::nested_trait::Answer::$patina_witness'/0]).

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

bump(_self@0_0) ->
    _self@0_0 +
        'Elixir.Patina.NestedFns':'from_impl::base'().

'nested_fns::nested_trait::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::nested_trait::Answer::answer'/2},
     {},
     {}}.
