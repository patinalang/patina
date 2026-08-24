-file("tests/ui/src/flow_control/early_return.pt", 1).
-module('Elixir.Patina.EarlyReturn').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         is_negative/1,
         is_negative_implicit/1,
         closure_value/1,
         expression_value/1,
         match_stopped/0,
         match_continued/0,
         match_scrutinee_value/1,
         nested_value/1,
         if_condition_value/1]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
mark(_value@0_0) ->
    begin
        io:put_chars([[io_lib:format("~ktp", [_value@0_0])]]),
        io:nl()
    end,
    _value@0_0.
match_value(_choice@0_0) ->
    _value@1_0 =
        case _choice@0_0 of
            'Elixir.Patina.EarlyReturn.Choice.Stop' ->
                '$patina_marker':return(10);
            'Elixir.Patina.EarlyReturn.Choice.Continue' ->
                20
        end,
    _value@1_0 + 1.
is_negative(_x@0_0) ->
    case _x@0_0 >= 0 of
        true ->
            '$patina_marker':return(false);
        false ->
            {}
    end,
    '$patina_marker':return(true).
is_negative_implicit(_x@0_0) ->
    case _x@0_0 >= 0 of
        true ->
            '$patina_marker':return(false);
        false ->
            {}
    end,
    true.
closure_value(_stop@0_0) ->
    __closure@1_0 =
        fun() ->
               case _stop@0_0 of
                   true ->
                       '$patina_marker':return(4);
                   false ->
                       {}
               end,
               5
        end,
    10.
expression_value(_stop@0_0) ->
    mark(1)
    +
    begin
        case _stop@0_0 of
            true ->
                '$patina_marker':return(mark(2));
            false ->
                {}
        end,
        mark(3)
    end.
match_stopped() ->
    match_value('Elixir.Patina.EarlyReturn.Choice.Stop').
match_continued() ->
    match_value('Elixir.Patina.EarlyReturn.Choice.Continue').
match_scrutinee_value(_stop@0_0) ->
    case
        begin
            case _stop@0_0 of
                true ->
                    '$patina_marker':return(40);
                false ->
                    {}
            end,
            'Elixir.Patina.EarlyReturn.Choice.Continue'
        end
    of
        'Elixir.Patina.EarlyReturn.Choice.Stop' ->
            50;
        'Elixir.Patina.EarlyReturn.Choice.Continue' ->
            60
    end.
nested_value(_x@0_0) ->
    _result@1_0 =
        begin
            case _x@0_0 =:= 0 of
                true ->
                    '$patina_marker':return(10);
                false ->
                    {}
            end,
            begin
                case _x@0_0 =:= 1 of
                    true ->
                        '$patina_marker':return(20);
                    false ->
                        {}
                end,
                30
            end
        end,
    _result@1_0 + 1.
if_condition_value(_stop@0_0) ->
    case
        begin
            case _stop@0_0 of
                true ->
                    '$patina_marker':return(10);
                false ->
                    {}
            end,
            true
        end
    of
        true ->
            20;
        false ->
            30
    end.
