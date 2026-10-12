-file("tests/ui/src/fn/nested_fns.pt", 87).

-module('Elixir.Patina.NestedFns.Wrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         trait_in_method/1,
         'NestedFns::{impl#1}::trait_in_method::Base::$patina_witness'/0,
         'NestedFns::first_answer::Answer::$patina_witness'/0,
         'NestedFns::second_answer::Answer::$patina_witness'/0,
         'NestedFns::answers_in_blocks::Answer::$patina_witness'/0,
         'NestedFns::answers_in_blocks::Answer#1::$patina_witness'/0]).

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
    'Elixir.Patina.NestedFns':'NestedFns::{impl#1}::trait_in_method::Base::base'('Elixir.Patina.NestedFns.Wrapper':'NestedFns::{impl#1}::trait_in_method::Base::$patina_witness'(),
                                                                                 _self@0_0).

'NestedFns::{impl#1}::trait_in_method::Base::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'NestedFns::{impl#1}::trait_in_method::Base::base'/2},
     {},
     {}}.

'NestedFns::first_answer::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'NestedFns::first_answer::Answer::answer'/2},
     {},
     {}}.

'NestedFns::second_answer::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'NestedFns::second_answer::Answer::answer'/2},
     {},
     {}}.

'NestedFns::answers_in_blocks::Answer::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'NestedFns::answers_in_blocks::Answer::answer'/2},
     {},
     {}}.

'NestedFns::answers_in_blocks::Answer#1::$patina_witness'() ->
    {{fun 'Elixir.Patina.NestedFns':'NestedFns::answers_in_blocks::Answer#1::answer'/2},
     {},
     {}}.
