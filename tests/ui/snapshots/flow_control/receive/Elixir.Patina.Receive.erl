-file("tests/ui/src/flow_control/receive.pt", 1).

-module('Elixir.Patina.Receive').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         take_int/0,
         take_int_or_timeout/0,
         take_atom/0,
         echo/0,
         sum_until_stop/0,
         unsafe_value/0,
         only_timeout/0,
         guarded_or/0,
         merge_timeout/0,
         unchanged_timeout/0,
         durations/0,
         contextual_after/0,
         raw_identifiers/0,
         closure_receive/1,
         diverging_timeout/0]).

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

take_int() ->
    receive
        _n@0_0 when erlang:is_integer(_n@0_0) -> _n@0_0
    end.

take_int_or_timeout() ->
    receive
        _n@0_0 when erlang:is_integer(_n@0_0) -> _n@0_0
        after 'Elixir.Patina.Core.Timeout':from_millis(10) -> -1
    end.

take_atom() ->
    receive
        _a@0_0 when erlang:is_atom(_a@0_0) -> _a@0_0
    end.

echo() ->
    receive
        {ping, _from@0_0} when erlang:is_pid(_from@0_0) ->
            erlang:send(_from@0_0, pong),
            {}
    end.

sum_until_stop() ->
    _total@0_0 = 0,
    case fun _$patina_loop(_total@0_0) ->
                 _ = begin
                         receive
                             _n@1_0 when erlang:is_integer(_n@1_0) ->
                                 _total@0_1 = _total@0_0 + _n@1_0,
                                 {};
                             stop ->
                                 _branch_value@2_0 = begin
                                                         '$patina_marker':break({break,
                                                                                 0,
                                                                                 {},
                                                                                 _total@0_0}),
                                                         []
                                                     end,
                                 _total@0_1 = _total@0_0,
                                 _branch_value@2_0
                         end
                     end,
                 _$patina_loop(_total@0_1)
         end(_total@0_0)
        of
        {break, 0, _loop_value@3_0, _total@0_1} ->
            _loop_value@3_0
    end,
    _total@0_1.

unsafe_value() ->
    begin
        receive
            {value, _number@0_0}
                when erlang:is_integer(_number@0_0) ->
                _number@0_0
            after 'Elixir.Patina.Core.Timeout':from_millis(0) -> -1
        end
    end.

only_timeout() ->
    receive
         after 'Elixir.Patina.Core.Timeout':from_millis(0) -> 7
    end.

guarded_or() ->
    receive
        {left, _n@0_0}
            when erlang:is_integer(_n@0_0), _n@0_0 > 0 ->
            _n@0_0;
        {right, _n@0_0}
            when erlang:is_integer(_n@0_0), _n@0_0 > 0 ->
            _n@0_0
    end.

merge_timeout() ->
    _value@0_0 = 1,
    receive
        _n@1_0 when erlang:is_integer(_n@1_0) ->
            _value@0_1 = _n@1_0,
            {}
        after 'Elixir.Patina.Core.Timeout':from_millis(0) ->
                  _value@0_1 = 2, {}
    end,
    _value@0_1.

unchanged_timeout() ->
    _value@0_0 = 1,
    receive
        _n@1_0 when erlang:is_integer(_n@1_0) ->
            _value@0_1 = _n@1_0,
            {}
        after 'Elixir.Patina.Core.Timeout':from_millis(0) ->
                  _branch_value@2_0 = begin {} end,
                  _value@0_1 = _value@0_0,
                  _branch_value@2_0
    end,
    _value@0_1.

durations() ->
    begin
        _element@0_0 =
            'Elixir.Patina.Core.Timeout':from_seconds(2),
        _element@1_0 =
            'Elixir.Patina.Core.Timeout':from_minutes(3),
        {_element@0_0, _element@1_0}
    end.

contextual_after() ->
    _after@0_0 = 4,
    _after@0_0.

raw_identifiers() ->
    _value@0_0 = 9,
    _value@0_0.

closure_receive(_value@0_0) ->
    _take@2_0 = fun () ->
                        receive
                            _n@1_0 when erlang:is_integer(_n@1_0) ->
                                _n@1_0 + _value@0_0
                        end
                end,
    _take@2_0().

diverging_timeout() ->
    receive
        _n@0_0 when erlang:is_integer(_n@0_0) ->
            '$patina_marker':return(_n@0_0),
            []
        after 'Elixir.Patina.Core.Timeout':from_millis(0) ->
                  begin '$patina_marker':return(-1), [] end
    end.
