-file("tests/ui/src/fn/backtick_ident.pt", 1).

-module('Elixir.Patina.BacktickIdent').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'exists?'/1,
         'save!'/0,
         'a{b}'/0,
         type/0,
         main/0]).

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

'exists?'(_x@0_0) -> _x@0_0 > 0.

'save!'() ->
    io:put_chars([[<<"saved!"/utf8>>]]),
    io:nl(),
    {}.

'a{b}'() -> 1.

type() -> 2.

main() ->
    case [] of
        {'$patina_label_0'} -> {};
        _ ->
            _s@0_0 = 1,
            _ = type(),
            _value?@1_0 = true,
            _type@2_0 = 2,
            case not
                     'Elixir.Patina.BacktickIdent.S':'valid?'(_s@0_0)
                of
                true ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _ -> {}
            end,
            case not
                     'Elixir.Patina.BacktickIdent.S':'valid?'(_s@0_0)
                of
                true ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _ -> {}
            end,
            case not
                     'Elixir.Patina.BacktickIdent.S':check_via_self(_s@0_0)
                of
                true ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _ -> {}
            end,
            case not 'exists?'(1) of
                true ->
                    '$patina_marker':jump('$patina_label_0'),
                    [];
                _ -> {}
            end,
            'exists?'(1),
            'save!'(),
            io:put_chars([[<<"`a{b}`"/utf8>>]]),
            io:nl(),
            io:put_chars(begin
                             _element@4_0 = begin
                                                _element@3_0 =
                                                    io_lib:format("~ktp",
                                                                  [_value?@1_0]),
                                                [_element@3_0]
                                            end,
                             [_element@4_0]
                         end),
            io:nl(),
            io:put_chars(begin
                             _element@6_0 = begin
                                                _element@5_0 =
                                                    io_lib:format("~ktp",
                                                                  [_type@2_0]),
                                                [_element@5_0]
                                            end,
                             [_element@6_0]
                         end),
            io:nl(),
            _ = 'a{b}'(),
            {}
    end.
