-file("tests/ui/src/variable_bindings/deferred_initi"
      "alization.pt",
      1).

-module('Elixir.Patina.DeferredInitialization').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, '$patina_loop_6_28'/2, main/0]).

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

initialized_in_branches(_condition@0_0) ->
    begin _value@1_0 = {} end,
    case _condition@0_0 of
        true ->
            _value@1_1 = 1,
            {};
        false ->
            _value@1_1 = 2,
            {}
    end,
    _value@1_1.

initialized_after_diverging_branch(_condition@0_0) ->
    begin _value@1_0 = {} end,
    case _condition@0_0 of
        true ->
            _value@1_1 = 1,
            '$patina_marker':return(_value@1_1),
            {};
        false ->
            _branch_value@2_0 = {},
            _value@1_1 = _value@1_0,
            _branch_value@2_0
    end,
    _value@1_2 = 2,
    _value@1_2.

'$patina_loop_6_28'(_condition@0_0, _value@1_0) ->
    _ = begin
            case _condition@0_0 of
                true ->
                    _value@1_1 = 1,
                    '$patina_marker':break({break, 0, {}, _value@1_1}),
                    {};
                false ->
                    _value@1_1 = 2,
                    '$patina_marker':break({break, 0, {}, _value@1_1}),
                    {}
            end
        end,
    '$patina_marker':continue(_condition@0_0, _value@1_1).

initialized_on_every_loop_exit(_condition@0_0) ->
    begin _value@1_0 = {} end,
    case '$patina_loop_6_28'(_condition@0_0, _value@1_0) of
        {break, 0, _loop_value@2_0, _value@1_1} ->
            _loop_value@2_0
    end,
    _value@1_1.

main() ->
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [initialized_in_branches(true)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [initialized_after_diverging_branch(false)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[io_lib:format("~ktp",
                                     [initialized_on_every_loop_exit(true)])]]),
        io:nl(),
        {}
    end.
