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
         generic/0,
         in_closure/0,
         as_value/0,
         not_exported/0,
         from_impl/0,
         'from_impl::base'/0,
         nested_trait/0,
         'nested_fns::nested_trait::Answer::answer'/2,
         'nested_fns::{impl#1}::trait_in_method::Base::base'/2,
         trait_in_method/0,
         first_answer/0,
         'nested_fns::first_answer::Answer::answer'/2,
         second_answer/0,
         'nested_fns::second_answer::Answer::answer'/2,
         answers_in_blocks/0,
         'nested_fns::answers_in_blocks::Answer::answer'/2,
         'nested_fns::answers_in_blocks::Answer#1::answer'/2]).

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
        _ -> 'mutual::is_odd'(_n@0_0 - 1)
    end.

'mutual::is_odd'(_n@0_0) ->
    case _n@0_0 =:= 0 of
        true -> false;
        _ -> 'mutual::is_even'(_n@0_0 - 1)
    end.

first() -> 'first::helper'().

'first::helper'() -> 1.

second() -> 'second::helper'().

'second::helper'() -> 2.

shadowed_blocks() ->
    _a@0_0 = 'shadowed_blocks::f'(),
    _b@1_0 = 'shadowed_blocks::f#1'(),
    _a@0_0 + _b@1_0.

'shadowed_blocks::f'() -> 10.

'shadowed_blocks::f#1'() -> 20.

recursive_loop(_n@0_0) ->
    'recursive_loop::sum_to'(_n@0_0).

'recursive_loop::sum_to'(_n@0_0) ->
    _total@1_0 = 0,
    _iter@3_0 = begin
                    _self@2_0 =
                        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(0,
                                                                                _n@0_0),
                    _self@2_0
                end,
    {_iter@3_1, _total@1_1} = fun _$patina_loop(_iter@3_0,
                                                _total@1_0) ->
                                      case [] of
                                          {'$patina_label_0',
                                           _iter@3_2,
                                           _total@1_3} ->
                                              {_iter@3_2, _total@1_3};
                                          _ ->
                                              case [] of
                                                  {'$patina_label_1',
                                                   _iter@3_1,
                                                   _total@1_1} ->
                                                      _branch_value@14_0 = {},
                                                      _total@1_2 = _total@1_1,
                                                      _branch_value@14_0;
                                                  _ ->
                                                      case begin
                                                               _self@4_0 =
                                                                   _iter@3_0,
                                                               case
                                                                   _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                       <
                                                                       _self@4_0#'Elixir.Patina.Core.Range':'Range'.'end'
                                                                   of
                                                                   true ->
                                                                       _0@8_0 =
                                                                           begin
                                                                               _element@5_0 =
                                                                                   _self@4_0#'Elixir.Patina.Core.Range':'Range'.start,
                                                                               _element@7_0 =
                                                                                   begin
                                                                                       _start@6_0 =
                                                                                           _self@4_0#'Elixir.Patina.Core.Range':'Range'.start
                                                                                               +
                                                                                               1,
                                                                                       _self@4_0#'Elixir.Patina.Core.Range':'Range'{start
                                                                                                                                        =
                                                                                                                                        _start@6_0}
                                                                                   end,
                                                                               {_element@5_0,
                                                                                _element@7_0}
                                                                           end,
                                                                       _0@8_0;
                                                                   _ -> nil
                                                               end
                                                           end
                                                          of
                                                          nil ->
                                                              _branch_value@13_0 =
                                                                  begin
                                                                      '$patina_marker':jump('$patina_label_0',
                                                                                            _iter@3_0,
                                                                                            _total@1_0),
                                                                      []
                                                                  end,
                                                              _iter@3_1 =
                                                                  _iter@3_0,
                                                              _total@1_2 =
                                                                  _total@1_0,
                                                              _branch_value@13_0;
                                                          _value@11_0 = {_i@9_0,
                                                                         _next@10_0}
                                                              when
                                                                erlang:is_tuple(_value@11_0)
                                                                    andalso
                                                                    erlang:tuple_size(_value@11_0)
                                                                        =:= 2 ->
                                                              _iter@3_1 =
                                                                  _next@10_0,
                                                              case _i@9_0 =:= 3
                                                                  of
                                                                  true ->
                                                                      _total@1_1 =
                                                                          _total@1_0
                                                                              +
                                                                              'recursive_loop::sum_to'(2),
                                                                      '$patina_marker':jump('$patina_label_1',
                                                                                            _iter@3_1,
                                                                                            _total@1_1),
                                                                      [];
                                                                  _ ->
                                                                      _branch_value@12_0 =
                                                                          {},
                                                                      _total@1_1 =
                                                                          _total@1_0,
                                                                      _branch_value@12_0
                                                              end,
                                                              _total@1_2 =
                                                                  _total@1_1 +
                                                                      _i@9_0,
                                                              {}
                                                      end
                                              end,
                                              _$patina_loop(_iter@3_1,
                                                            _total@1_2)
                                      end
                              end(_iter@3_0, _total@1_0),
    _total@1_1.

generic() ->
    _witness_Show@0_0 =
        'Elixir.Patina.NestedFns.$impl.Core.Int':'nested_fns::Show::$patina_witness'(),
    _witnesses_where_describe@1_0 = {_witness_Show@0_0},
    'generic::describe'(_witnesses_where_describe@1_0, 5).

'generic::describe'(_witnesses_where_describe@0_0,
                    _value@1_0) ->
    _witness_Show@2_0 = erlang:element(1,
                                       _witnesses_where_describe@0_0),
    _vtable_Show@3_0 = erlang:element(1, _witness_Show@2_0),
    _fn_Show_show@4_0 = erlang:element(1, _vtable_Show@3_0),
    _fn_Show_show@4_0(_witness_Show@2_0, _value@1_0).

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
    _witness_Answer@0_0 =
        'Elixir.Patina.NestedFns.Counter':'nested_fns::nested_trait::Answer::$patina_witness'(),
    'nested_fns::nested_trait::Answer::answer'(_witness_Answer@0_0,
                                               0).

'nested_fns::nested_trait::Answer::answer'(_witness_Answer@0_0,
                                           _self@1_0) ->
    42.

'nested_fns::{impl#1}::trait_in_method::Base::base'(_witness_Base@0_0,
                                                    _self@1_0) ->
    1000.

trait_in_method() ->
    'Elixir.Patina.NestedFns.Wrapper':trait_in_method('Elixir.Patina.NestedFns.Wrapper').

first_answer() ->
    _witness_Answer@0_0 =
        'Elixir.Patina.NestedFns.Wrapper':'nested_fns::first_answer::Answer::$patina_witness'(),
    'nested_fns::first_answer::Answer::answer'(_witness_Answer@0_0,
                                               'Elixir.Patina.NestedFns.Wrapper').

'nested_fns::first_answer::Answer::answer'(_witness_Answer@0_0,
                                           _self@1_0) ->
    1.

second_answer() ->
    _witness_Answer@0_0 =
        'Elixir.Patina.NestedFns.Wrapper':'nested_fns::second_answer::Answer::$patina_witness'(),
    'nested_fns::second_answer::Answer::answer'(_witness_Answer@0_0,
                                                'Elixir.Patina.NestedFns.Wrapper').

'nested_fns::second_answer::Answer::answer'(_witness_Answer@0_0,
                                            _self@1_0) ->
    2.

answers_in_blocks() ->
    _witness_Answer@0_0 =
        'Elixir.Patina.NestedFns.Wrapper':'nested_fns::answers_in_blocks::Answer::$patina_witness'(),
    _witness_Answer@1_0 =
        'Elixir.Patina.NestedFns.Wrapper':'nested_fns::answers_in_blocks::Answer#1::$patina_witness'(),
    _a@2_0 =
        'nested_fns::answers_in_blocks::Answer::answer'(_witness_Answer@0_0,
                                                        'Elixir.Patina.NestedFns.Wrapper'),
    _b@3_0 =
        'nested_fns::answers_in_blocks::Answer#1::answer'(_witness_Answer@1_0,
                                                          'Elixir.Patina.NestedFns.Wrapper'),
    _a@2_0 * 10 + _b@3_0.

'nested_fns::answers_in_blocks::Answer::answer'(_witness_Answer@0_0,
                                                _self@1_0) ->
    3.

'nested_fns::answers_in_blocks::Answer#1::answer'(_witness_Answer@0_0,
                                                  _self@1_0) ->
    4.
