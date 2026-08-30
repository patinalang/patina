-file("tests/ui/src/fn/backtick_ident.pt", 1).
-module('Elixir.Patina.BacktickIdent').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,'exists?'/1,'save!'/0,'a{b}'/0,type/0,main/0]).
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
'exists?'(_x@0_0) ->
    _x@0_0 > 0.
'save!'() ->
    begin
        io:put_chars([[<<"saved!">>]]),
        io:nl()
    end.
'a{b}'() ->
    1.
type() ->
    2.
main() ->
    _s@0_0 =
        ('Elixir.Patina.BacktickIdent.S':'__struct__'())#{value := 1},
    _ = type(),
    _value?@1_0 = true,
    _type@2_0 = 2,
    case not 'Elixir.Patina.BacktickIdent.S':'valid?'(_s@0_0) of
        true ->
            '$patina_marker':return({});
        false ->
            {}
    end,
    case not 'Elixir.Patina.BacktickIdent.S':'valid?'(_s@0_0) of
        true ->
            '$patina_marker':return({});
        false ->
            {}
    end,
    case not 'Elixir.Patina.BacktickIdent.S':check_via_self(_s@0_0) of
        true ->
            '$patina_marker':return({});
        false ->
            {}
    end,
    case not 'exists?'(1) of
        true ->
            '$patina_marker':return({});
        false ->
            {}
    end,
    'exists?'(1),
    'save!'(),
    begin
        io:put_chars([[<<"`a{b}`">>]]),
        io:nl()
    end,
    begin
        io:put_chars([[io_lib:format("~ktp", [_value?@1_0])]]),
        io:nl()
    end,
    begin
        io:put_chars([[io_lib:format("~ktp", [_type@2_0])]]),
        io:nl()
    end,
    _ = 'a{b}'().
