-file("tests/ui/src/fn/nested_fns.pt", 87).

-module('Elixir.Patina.NestedFns.Wrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         trait_in_method/1,
         'nested_fns::{impl#1}::trait_in_method::Base::$patina_witness'/0,
         'nested_fns::first_answer::Answer::$patina_witness'/0,
         'nested_fns::second_answer::Answer::$patina_witness'/0,
         'nested_fns::answers_in_blocks::Answer::$patina_witness'/0,
         'nested_fns::answers_in_blocks::Answer#1::$patina_witness'/0]).

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

trait_in_method(_self@0_0) ->
    _witness_Base@1_0 =
        'nested_fns::{impl#1}::trait_in_method::Base::$patina_witness'(),
    'Elixir.Patina.NestedFns':'nested_fns::{impl#1}::trait_in_method::Base::base'(_witness_Base@1_0,
                                                                                  _self@0_0).

'nested_fns::{impl#1}::trait_in_method::Base::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::{impl#1}::trait_in_method::Base::base'/2},
     {},
     {}}.

'nested_fns::first_answer::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::first_answer::Answer::answer'/2},
     {},
     {}}.

'nested_fns::second_answer::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::second_answer::Answer::answer'/2},
     {},
     {}}.

'nested_fns::answers_in_blocks::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::answers_in_blocks::Answer::answer'/2},
     {},
     {}}.

'nested_fns::answers_in_blocks::Answer#1::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'nested_fns::answers_in_blocks::Answer#1::answer'/2},
     {},
     {}}.
