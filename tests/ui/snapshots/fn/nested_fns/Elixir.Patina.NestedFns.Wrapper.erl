-file("tests/ui/src/fn/nested_fns.pt", 87).

-module('Elixir.Patina.NestedFns.Wrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         trait_in_method/1,
         'NestedFns::first_answer::Answer::answer'/2,
         'NestedFns::second_answer::Answer::answer'/2,
         'NestedFns::answers_in_blocks::Answer::answer'/2,
         'NestedFns::answers_in_blocks::Answer#1::answer'/2]).

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
    'NestedFns::{impl#1}::trait_in_method::Base::base'({'impl NestedFns::{impl#1}::trait_in_method::Base for NestedFns::Wrapper',
                                                        'Elixir.Patina.NestedFns.Wrapper'},
                                                       _self@0_0).

'NestedFns::{impl#1}::trait_in_method::Base::base'(_witness@0_0 =
                                                       {'impl NestedFns::{impl#1}::trait_in_method::Base for NestedFns::Wrapper',
                                                        _},
                                                   _arg@1_0) ->
    'Elixir.Patina.NestedFns':'NestedFns::{impl#1}::trait_in_method::Base::base'(_witness@0_0,
                                                                                 _arg@1_0).

'NestedFns::first_answer::Answer::answer'(_witness@0_0 =
                                              {'impl NestedFns::first_answer::Answer for NestedFns::Wrapper',
                                               _},
                                          _arg@1_0) ->
    'Elixir.Patina.NestedFns':'NestedFns::first_answer::Answer::answer'(_witness@0_0,
                                                                        _arg@1_0).

'NestedFns::second_answer::Answer::answer'(_witness@0_0 =
                                               {'impl NestedFns::second_answer::Answer for NestedFns::Wrapper',
                                                _},
                                           _arg@1_0) ->
    'Elixir.Patina.NestedFns':'NestedFns::second_answer::Answer::answer'(_witness@0_0,
                                                                         _arg@1_0).

'NestedFns::answers_in_blocks::Answer::answer'(_witness@0_0 =
                                                   {'impl NestedFns::answers_in_blocks::Answer for NestedFns::Wrapper',
                                                    _},
                                               _arg@1_0) ->
    'Elixir.Patina.NestedFns':'NestedFns::answers_in_blocks::Answer::answer'(_witness@0_0,
                                                                             _arg@1_0).

'NestedFns::answers_in_blocks::Answer#1::answer'(_witness@0_0 =
                                                     {'impl NestedFns::answers_in_blocks::Answer#1 for NestedFns::Wrapper',
                                                      _},
                                                 _arg@1_0) ->
    'Elixir.Patina.NestedFns':'NestedFns::answers_in_blocks::Answer#1::answer'(_witness@0_0,
                                                                               _arg@1_0).
