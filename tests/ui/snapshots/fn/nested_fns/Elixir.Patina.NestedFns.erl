-file("tests/ui/src/fn/nested_fns.pt", 1).

-module('Elixir.Patina.NestedFns').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         before_declaration/0,
         mutual/1,
         first/0,
         second/0,
         shadowed_blocks/0,
         recursive_loop/1,
         'recursive_loop::sum_to$patina_loop_1'/2,
         generic/0,
         in_closure/0,
         as_value/0,
         not_exported/0,
         from_impl/0,
         'from_impl::base'/0,
         nested_trait/0,
         'NestedFns::nested_trait::Answer::answer'/2,
         'NestedFns::{impl#1}::trait_in_method::Base::base'/2,
         trait_in_method/0,
         first_answer/0,
         'NestedFns::first_answer::Answer::answer'/2,
         second_answer/0,
         'NestedFns::second_answer::Answer::answer'/2,
         answers_in_blocks/0,
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

before_declaration() ->
    _value@0_0 = 'before_declaration::later'(),
    _value@0_0.

'before_declaration::later'() -> 7.

mutual(_n@0_0) -> 'mutual::is_even'(_n@0_0).

'mutual::is_even'(_n@0_0) ->
    case _n@0_0 =:= 0 of
        true -> true;
        false -> 'mutual::is_odd'(_n@0_0 - 1)
    end.

'mutual::is_odd'(_n@0_0) ->
    case _n@0_0 =:= 0 of
        true -> false;
        false -> 'mutual::is_even'(_n@0_0 - 1)
    end.

first() -> 'first::helper'().

'first::helper'() -> 1.

second() -> 'second::helper'().

'second::helper'() -> 2.

shadowed_blocks() ->
    _a@0_0 = begin 'shadowed_blocks::f'() end,
    _b@1_0 = begin 'shadowed_blocks::f#1'() end,
    _a@0_0 + _b@1_0.

'shadowed_blocks::f'() -> 10.

'shadowed_blocks::f#1'() -> 20.

recursive_loop(_n@0_0) ->
    'recursive_loop::sum_to'(_n@0_0).

'recursive_loop::sum_to$patina_loop_1'(_iter@2_0,
                                       _total@1_0) ->
    _ = begin
            case
                'Elixir.Patina.Core.Range':'Core::Iterator::next'({'impl Core::Iterator for Core::Range<int>',
                                                                   'Elixir.Patina.Core.Range'},
                                                                  _iter@2_0)
                of
                nil ->
                    _branch_value@7_0 = '$patina_marker':break({break,
                                                                0,
                                                                {},
                                                                _iter@2_0,
                                                                _total@1_0}),
                    _iter@2_1 = _iter@2_0,
                    _total@1_2 = _total@1_0,
                    _branch_value@7_0;
                _value@5_0 = {_i@3_0, _next@4_0}
                    when erlang:is_tuple(_value@5_0) andalso
                             erlang:tuple_size(_value@5_0) =:= 2 ->
                    _iter@2_1 = _next@4_0,
                    begin
                        case _i@3_0 =:= 3 of
                            true ->
                                _total@1_1 = _total@1_0 +
                                                 'recursive_loop::sum_to'(2),
                                '$patina_marker':continue(_iter@2_1,
                                                          _total@1_1);
                            false ->
                                _branch_value@6_0 = {},
                                _total@1_1 = _total@1_0,
                                _branch_value@6_0
                        end,
                        _total@1_2 = _total@1_1 + _i@3_0,
                        {}
                    end
            end
        end,
    '$patina_marker':continue(_iter@2_1, _total@1_2).

'recursive_loop::sum_to'(_n@0_0) ->
    _total@1_0 = 0,
    case
        'Elixir.Patina.Core.$impl':'Core::IntoIterator::into_iter'({'impl Core::IntoIterator for _',
                                                                    'Elixir.Patina.Core.$impl',
                                                                    {'impl Core::Iterator for Core::Range<int>',
                                                                     'Elixir.Patina.Core.Range'}},
                                                                   'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                                                                           _n@0_0))
        of
        _iter@2_0 ->
            case 'recursive_loop::sum_to$patina_loop_1'(_iter@2_0,
                                                        _total@1_0)
                of
                {break, 0, _loop_value@8_0, _iter@2_1, _total@1_1} ->
                    _loop_value@8_0
            end
    end,
    _total@1_1.

generic() ->
    'generic::describe'({{'impl NestedFns::Show for int',
                          'Elixir.Patina.NestedFns.$impl.Core.Int'}},
                        5).

'generic::describe'(_witness@1_0 = {_witness@0_0},
                    _value@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'NestedFns::Show::show'(_witness@0_0,
                                                           _value@2_0).

in_closure() ->
    _f@1_0 = fun (_x@0_0) ->
                     'in_closure::{closure#0}::double'(_x@0_0)
             end,
    _f@1_0(21).

'in_closure::{closure#0}::double'(_x@0_0) -> _x@0_0 * 2.

as_value() ->
    _f@0_0 = fun 'as_value::triple'/1,
    'as_value::triple'(4).

'as_value::triple'(_x@0_0) -> _x@0_0 * 3.

not_exported() -> 'not_exported::hidden'().

'not_exported::hidden'() -> 5.

from_impl() ->
    'Elixir.Patina.NestedFns.Counter':bump(1).

'from_impl::base'() -> 100.

nested_trait() ->
    'NestedFns::nested_trait::Answer::answer'({'impl NestedFns::nested_trait::Answer for NestedFns::Counter',
                                               'Elixir.Patina.NestedFns.Counter'},
                                              0).

'NestedFns::nested_trait::Answer::answer'(_witness@0_0,
                                          _self@1_0) ->
    42.

'NestedFns::{impl#1}::trait_in_method::Base::base'(_witness@0_0,
                                                   _self@1_0) ->
    1000.

trait_in_method() ->
    'Elixir.Patina.NestedFns.Wrapper':trait_in_method('Elixir.Patina.NestedFns.Wrapper').

first_answer() ->
    'NestedFns::first_answer::Answer::answer'({'impl NestedFns::first_answer::Answer for NestedFns::Wrapper',
                                               'Elixir.Patina.NestedFns.Wrapper'},
                                              'Elixir.Patina.NestedFns.Wrapper').

'NestedFns::first_answer::Answer::answer'(_witness@0_0,
                                          _self@1_0) ->
    1.

second_answer() ->
    'NestedFns::second_answer::Answer::answer'({'impl NestedFns::second_answer::Answer for NestedFns::Wrapper',
                                                'Elixir.Patina.NestedFns.Wrapper'},
                                               'Elixir.Patina.NestedFns.Wrapper').

'NestedFns::second_answer::Answer::answer'(_witness@0_0,
                                           _self@1_0) ->
    2.

answers_in_blocks() ->
    _a@0_0 = begin
                 'NestedFns::answers_in_blocks::Answer::answer'({'impl NestedFns::answers_in_blocks::Answer for NestedFns::Wrapper',
                                                                 'Elixir.Patina.NestedFns.Wrapper'},
                                                                'Elixir.Patina.NestedFns.Wrapper')
             end,
    _b@1_0 = begin
                 'NestedFns::answers_in_blocks::Answer#1::answer'({'impl NestedFns::answers_in_blocks::Answer#1 for NestedFns::Wrapper',
                                                                   'Elixir.Patina.NestedFns.Wrapper'},
                                                                  'Elixir.Patina.NestedFns.Wrapper')
             end,
    _a@0_0 * 10 + _b@1_0.

'NestedFns::answers_in_blocks::Answer::answer'(_witness@0_0,
                                               _self@1_0) ->
    3.

'NestedFns::answers_in_blocks::Answer#1::answer'(_witness@0_0,
                                                 _self@1_0) ->
    4.
