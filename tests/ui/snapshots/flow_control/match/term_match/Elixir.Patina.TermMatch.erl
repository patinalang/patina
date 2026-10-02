-file("tests/ui/src/flow_control/match/term_match.pt",
      1).

-module('Elixir.Patina.TermMatch').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         literals/1,
         tuple/1,
         list_match/1,
         map/1,
         pinned/2,
         guarded/1,
         inferred_class_scrutinee/0,
         if_let_term/1,
         let_else_term/1,
         closure_term/1,
         literal_or/1,
         nested_or/1,
         int_class/1,
         float_class/1,
         atom_class/1,
         list_class/1,
         map_class/1,
         tuple_class/1,
         fn_class/1,
         bits_class/1,
         bin_class/1,
         pid_class/1,
         port_class/1,
         ref_class/1,
         number/1,
         alias_list/1,
         nested/1,
         nested_list/1,
         nested_map/1,
         tuple_terms/1,
         struct_term/1,
         option_none/1,
         option_some/1,
         inferred_some/1,
         nested_some/1,
         point/1,
         point_rest/1,
         point_range/1,
         celsius/1,
         celsius_any/1,
         tagged/1,
         elixir/1,
         elixir_rest/1,
         nested_in_tuple/1,
         wrapped_term/1,
         wrapped_int/1,
         literal_or_point/1,
         raw_constructor/1,
         qualified_constructor/1]).

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

use_int(_value@0_0) -> _value@0_0 + 1.

literals(_value@0_0) ->
    case _value@0_0 of
        nil -> 1;
        1 -> 2;
        1.5 -> 3;
        <<104, 101, 108, 108, 111>> -> 4;
        true -> 5;
        42 -> 6;
        _ -> 0
    end.

tuple(_value@0_0) ->
    case _value@0_0 of
        {some, {1, _inner@1_0}}
            when erlang:is_integer(_inner@1_0) ->
            _inner@1_0;
        _ -> 0
    end.

list_match(_value@0_0) ->
    case _value@0_0 of
        [_head@2_0 | _tail@1_0 = _] -> {_head@2_0, _tail@1_0};
        _ -> empty
    end.

map(_value@0_0) ->
    case _value@0_0 of
        #{<<"k"/utf8>> := _found@1_0} -> _found@1_0;
        _ -> missing
    end.

pinned(_value@0_0, _expected@1_0) ->
    case _value@0_0 of
        _expected@1_0 -> true;
        _ -> false
    end.

guarded(_value@0_0) ->
    case _value@0_0 of
        _found@1_0
            when erlang:is_integer(_found@1_0), _found@1_0 =:= 42 ->
            true;
        _ -> false
    end.

inferred_class_scrutinee() ->
    _matcher@2_0 = fun (_value@0_0) ->
                           case _value@0_0 of
                               _value@1_0 when erlang:is_integer(_value@1_0) ->
                                   true;
                               _ -> false
                           end
                   end,
    _matcher@2_0(1).

if_let_term(_value@0_0) ->
    case _value@0_0 of
        {ok, _} -> true;
        _ -> false
    end.

let_else_term(_value@0_0) ->
    case _value@0_0 of
        [_ | _] -> {};
        _ -> '$patina_marker':return(false)
    end,
    true.

closure_term(_value@0_0) ->
    _matcher@1_0 = fun () ->
                           case _value@0_0 of
                               [_ | _] -> true;
                               _ -> false
                           end
                   end,
    _matcher@1_0().

literal_or(_value@0_0) ->
    case _value@0_0 of
        1 -> true;
        2 -> true;
        ok -> true;
        _ -> false
    end.

nested_or(_value@0_0) ->
    case _value@0_0 of
        {ok, _value@1_0 = 1}
            when erlang:is_integer(_value@1_0) ->
            true;
        {ok, _value@2_0 = 2}
            when erlang:is_integer(_value@2_0) ->
            true;
        {error, _value@1_0 = 1}
            when erlang:is_integer(_value@1_0) ->
            true;
        {error, _value@2_0 = 2}
            when erlang:is_integer(_value@2_0) ->
            true;
        _ -> false
    end.

int_class(_value@0_0) ->
    case _value@0_0 of
        _n@1_0 when erlang:is_integer(_n@1_0) -> _n@1_0 + 1;
        _ -> 0
    end.

float_class(_value@0_0) ->
    case _value@0_0 of
        _n@1_0 when erlang:is_float(_n@1_0) -> _n@1_0 + 1.0;
        _ -> 0.0
    end.

atom_class(_value@0_0) ->
    case _value@0_0 of
        _a@1_0 when erlang:is_atom(_a@1_0) -> _a@1_0;
        _ -> no
    end.

list_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = [_ | _] when erlang:is_list(_value@1_0) ->
            true;
        _ -> false
    end.

map_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = #{<<"k"/utf8>> := _}
            when erlang:is_map(_value@1_0) ->
            true;
        _ -> false
    end.

tuple_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_tuple(_value@1_0) -> true;
        _ -> false
    end.

fn_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_function(_value@1_0) -> true;
        _ -> false
    end.

bits_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_bitstring(_value@1_0) -> true;
        _ -> false
    end.

bin_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_binary(_value@1_0) -> true;
        _ -> false
    end.

pid_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_pid(_value@1_0) -> true;
        _ -> false
    end.

port_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_port(_value@1_0) -> true;
        _ -> false
    end.

ref_class(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_reference(_value@1_0) -> true;
        _ -> false
    end.

number(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 when erlang:is_integer(_value@1_0) -> true;
        _value@2_0 when erlang:is_float(_value@2_0) -> true;
        _ -> false
    end.

alias_list(_value@0_0) ->
    case _value@0_0 of
        _whole@1_0 = _value@4_0 = [_head@3_0 | _tail@2_0 = _]
            when erlang:is_list(_value@4_0) ->
            {_whole@1_0, _head@3_0, _tail@2_0};
        _ -> no
    end.

nested(_value@0_0) ->
    case _value@0_0 of
        {_a@1_0, _b@2_0}
            when erlang:is_integer(_a@1_0), erlang:is_atom(_b@2_0),
                 _a@1_0 > 0 andalso _b@2_0 =:= ok ->
            true;
        _ -> false
    end.

nested_list(_value@0_0) ->
    case _value@0_0 of
        [_head@1_0 | _]
            when erlang:is_integer(_head@1_0), _head@1_0 > 0 ->
            true;
        _ -> false
    end.

nested_map(_value@0_0) ->
    case _value@0_0 of
        #{<<"k"/utf8>> := _found@1_0}
            when erlang:is_float(_found@1_0), _found@1_0 > 0.0 ->
            true;
        _ -> false
    end.

tuple_terms(_value@0_0) ->
    case _value@0_0 of
        {_value@1_0, _value@2_0}
            when erlang:is_integer(_value@1_0),
                 erlang:is_atom(_value@2_0) ->
            true;
        _ -> false
    end.

struct_term(_value@0_0) ->
    case _value@0_0 of
        {wrapper, _value@1_0}
            when erlang:is_integer(_value@1_0) ->
            true;
        _ -> false
    end.

option_none(_value@0_0) ->
    begin
        case _value@0_0 of
            nil -> 1;
            nil -> 2;
            _ -> 0
        end
    end.

option_some(_value@0_0) ->
    begin
        case _value@0_0 of
            _value@2_0 = _n@1_0 when erlang:is_integer(_n@1_0) ->
                _n@1_0;
            _ -> 0
        end
    end.

inferred_some(_value@0_0) ->
    begin
        case _value@0_0 of
            _value@2_0 = _n@1_0 when erlang:is_integer(_n@1_0) ->
                use_int(_n@1_0);
            _ -> 0
        end
    end.

nested_some(_value@0_0) ->
    begin
        case _value@0_0 of
            {some, _value@2_0 = _n@1_0}
                when erlang:is_integer(_n@1_0) ->
                use_int(_n@1_0);
            _value@3_0 = _value@2_0 = _n@1_0
                when erlang:is_integer(_n@1_0),
                     not (_value@3_0 =:= nil) ->
                use_int(_n@1_0);
            _ -> 0
        end
    end.

point(_value@0_0) ->
    begin
        case _value@0_0 of
            #'Elixir.Patina.TermMatch.Point':'Point'{x = _x@1_0,
                                                     y = _y@2_0}
                when erlang:is_integer(_x@1_0),
                     erlang:is_integer(_y@2_0) ->
                _x@1_0 + _y@2_0;
            _ -> 0
        end
    end.

point_rest(_value@0_0) ->
    begin
        case _value@0_0 of
            #'Elixir.Patina.TermMatch.Point':'Point'{x = _x@1_0,
                                                     y = _value@2_0}
                when erlang:is_integer(_x@1_0),
                     erlang:is_integer(_value@2_0) ->
                _x@1_0;
            _ -> 0
        end
    end.

point_range(_value@0_0) ->
    begin
        case _value@0_0 of
            #'Elixir.Patina.TermMatch.Point':'Point'{x = _range@1_0,
                                                     y = _value@2_0}
                when erlang:is_integer(_range@1_0), _range@1_0 >= 1,
                     _range@1_0 =< 3, erlang:is_integer(_value@2_0) ->
                true;
            _ -> false
        end
    end.

celsius(_value@0_0) ->
    begin
        case _value@0_0 of
            _c@1_0 when erlang:is_float(_c@1_0) -> _c@1_0;
            _ -> 0.0
        end
    end.

celsius_any(_value@0_0) ->
    begin
        case _value@0_0 of
            _value@1_0 when erlang:is_float(_value@1_0) -> true;
            _ -> false
        end
    end.

tagged(_value@0_0) ->
    begin
        case _value@0_0 of
            {tagged, _value@1_0, _value@2_0}
                when erlang:is_integer(_value@1_0),
                     erlang:is_binary(_value@2_0) ->
                _value@1_0;
            _ -> 0
        end
    end.

elixir(_value@0_0) ->
    begin
        case _value@0_0 of
            #{'__struct__' := 'Elixir.Patina.TermMatch.ElixirPoint',
              x := _x@1_0, y := _y@2_0}
                when erlang:is_integer(_x@1_0),
                     erlang:is_integer(_y@2_0) ->
                _x@1_0 + _y@2_0;
            _ -> 0
        end
    end.

elixir_rest(_value@0_0) ->
    begin
        case _value@0_0 of
            #{'__struct__' := 'Elixir.Patina.TermMatch.ElixirPoint',
              x := _x@1_0, y := _value@2_0}
                when erlang:is_integer(_x@1_0),
                     erlang:is_integer(_value@2_0) ->
                _x@1_0;
            _ -> 0
        end
    end.

nested_in_tuple(_value@0_0) ->
    begin
        case _value@0_0 of
            {point,
             #'Elixir.Patina.TermMatch.Point':'Point'{x = _x@1_0,
                                                      y = _y@2_0}}
                when erlang:is_integer(_x@1_0),
                     erlang:is_integer(_y@2_0) ->
                _x@1_0 + _y@2_0;
            _ -> 0
        end
    end.

wrapped_term(_value@0_0) ->
    begin
        case _value@0_0 of
            {wrapper, {_value@1_0, _value@2_0}}
                when erlang:is_integer(_value@1_0),
                     erlang:is_atom(_value@2_0) ->
                true;
            _ -> false
        end
    end.

wrapped_int(_value@0_0) ->
    begin
        case _value@0_0 of
            {wrapper, _value@1_0}
                when erlang:is_integer(_value@1_0) ->
                true;
            _ -> false
        end
    end.

literal_or_point(_value@0_0) ->
    begin
        case _value@0_0 of
            0 -> true;
            #'Elixir.Patina.TermMatch.Point':'Point'{x = 0,
                                                     y = _value@1_0}
                when erlang:is_integer(_value@1_0) ->
                true;
            _ -> false
        end
    end.

raw_constructor(_value@0_0) ->
    case _value@0_0 of _inner@1_0 -> _inner@1_0 end.

qualified_constructor(_value@0_0) ->
    case _value@0_0 of _inner@1_0 -> _inner@1_0 end.
