-file("tests/ui/src/flow_control/pin_patterns.pt", 1).

-module('Elixir.Patina.PinPatterns').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         match_pin/2,
         if_let_pin/2,
         '$patina_loop_6_32'/3,
         while_let_pin/2,
         let_else_pin/2,
         or_pin/3,
         map_value_pin/2,
         alias_pin/2,
         closure_pin/2,
         mutable_closure_pin/2,
         '$patina_loop_15_19'/2,
         loop_rebound_pin/1,
         sibling_shadow_pin/3,
         guarded_pin/3,
         macro_pin/2]).

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

match_pin(_actual@0_0, _expected@1_0) ->
    case _actual@0_0 of
        _expected@1_0 -> true;
        _ -> false
    end.

if_let_pin(_actual@0_0, _expected@1_0) ->
    case _actual@0_0 of
        _expected@1_0 -> true;
        _ -> false
    end.

'$patina_loop_6_32'(_current@2_0, _expected@1_0,
                    _count@3_0) ->
    _ = begin
            case _current@2_0 of
                _expected@1_0 ->
                    _count@3_1 = _count@3_0 + 1,
                    _current@2_1 = _current@2_0 + 1,
                    {};
                _ ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _count@3_0,
                                                                    _current@2_0})
                                        end,
                    _count@3_1 = _count@3_0,
                    _current@2_1 = _current@2_0,
                    _branch_value@4_0
            end
        end,
    '$patina_marker':continue(_current@2_1,
                              _expected@1_0,
                              _count@3_1).

while_let_pin(_start@0_0, _expected@1_0) ->
    _current@2_0 = _start@0_0,
    _count@3_0 = 0,
    case '$patina_loop_6_32'(_current@2_0,
                             _expected@1_0,
                             _count@3_0)
        of
        {break, 0, _loop_value@5_0, _count@3_1, _current@2_1} ->
            _loop_value@5_0
    end,
    _count@3_1 * 10 + _current@2_1.

let_else_pin(_actual@0_0, _expected@1_0) ->
    case _actual@0_0 of
        _expected@1_0 -> {};
        _ ->
            '$patina_marker':return(false),
            {}
    end,
    true.

or_pin(_value@0_0, _first@1_0, _second@2_0) ->
    case _value@0_0 of
        _first@1_0 -> true;
        _second@2_0 -> true;
        _ -> false
    end.

map_value_pin(_map@0_0, _expected@1_0) ->
    case _map@0_0 of
        #{<<"key"/utf8>> := _expected@1_0} -> true;
        _ -> false
    end.

alias_pin(_value@0_0, _expected@1_0) ->
    case _value@0_0 of
        _matched@2_0 = _expected@1_0 -> _matched@2_0;
        _ -> -1
    end.

closure_pin(_actual@0_0, _expected@1_0) ->
    _matcher@2_0 = fun () ->
                           case _actual@0_0 of
                               _expected@1_0 -> true;
                               _ -> false
                           end
                   end,
    _matcher@2_0().

mutable_closure_pin(_actual@0_0, _initial@1_0) ->
    _expected@2_0 = _initial@1_0,
    _expected@2_1 = _expected@2_0 + 1,
    _matcher@3_0 = fun () ->
                           case _actual@0_0 of
                               _expected@2_1 -> true;
                               _ -> false
                           end
                   end,
    _matcher@3_0().

'$patina_loop_15_19'(_target@0_0, _expected@1_0) ->
    _ = begin
            case _target@0_0 of
                _expected@1_0 ->
                    _branch_value@2_0 = '$patina_marker':break({break,
                                                                0,
                                                                _expected@1_0,
                                                                _expected@1_0}),
                    _expected@1_1 = _expected@1_0,
                    _branch_value@2_0;
                _ -> _expected@1_1 = _expected@1_0 + 1
            end,
            {}
        end,
    '$patina_marker':continue(_target@0_0, _expected@1_1).

loop_rebound_pin(_target@0_0) ->
    _expected@1_0 = 1,
    case '$patina_loop_15_19'(_target@0_0, _expected@1_0) of
        {break, 0, _loop_value@3_0, _expected@1_1} ->
            _loop_value@3_0
    end.

sibling_shadow_pin(_first@0_0, _second@1_0,
                   _outer@2_0) ->
    _a@3_0 = _outer@2_0,
    case {_first@0_0, _second@1_0} of
        {_a@4_0, _a@3_0} -> _a@4_0;
        _ -> -1
    end.

guarded_pin(_value@0_0, _expected@1_0, _enabled@2_0) ->
    case _value@0_0 of
        _expected@1_0 when _enabled@2_0 -> true;
        _ -> false
    end.

macro_pin(_value@0_0, _expected@1_0) ->
    case _value@0_0 of
        _expected@1_0 -> true;
        _ -> false
    end.
