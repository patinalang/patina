-file("tests/ui/src/fn/backtick_ident.pt", 1).

-module('Elixir.Patina.BacktickIdent').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         out_of_line_keyword_module/0,
         keyword_module/0,
         keyword_extern/0,
         'exists?'/1,
         'save!'/0,
         'a{b}'/0,
         type/0,
         keyword_trait/0,
         main/0,
         keyword_nested_fns/0,
         keyword_local/0,
         keyword_literals/0,
         keyword_nested_traits/0,
         keyword_nested_mods/0,
         unit_pattern/0,
         'Self'/0]).

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

out_of_line_keyword_module() ->
    'Elixir.Patina.BacktickIdent.BacktickIdent.Pkg':value().

keyword_module() ->
    'Elixir.Patina.BacktickIdent.Pkg':value().

keyword_extern() -> erlang:self() =:= erlang:self().

'exists?'(_x@0_0) -> _x@0_0 > 0.

'save!'() ->
    io:put_chars([[<<"saved!"/utf8>>]]),
    io:nl(),
    {}.

'a{b}'() -> 1.

type() -> 2.

keyword_trait() ->
    _witness_Shape@0_0 =
        'Elixir.Patina.BacktickIdent.S':'backtick_ident::type::Shape::$patina_witness'(),
    'Elixir.Patina.BacktickIdent.S':'backtick_ident::type::Shape::fn'(_witness_Shape@0_0,
                                                                      43).

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

keyword_nested_fns() ->
    _first@0_0 = 'keyword_nested_fns::type'(),
    _second@1_0 = 'keyword_nested_fns::type#1'(),
    _first@0_0 + _second@1_0.

'keyword_nested_fns::type'() -> 1.

'keyword_nested_fns::type#1'() -> 2.

keyword_local() ->
    _type@0_0 = 4,
    _type@0_0 + 1.

keyword_literals() -> {<<"`fn`"/utf8>>, '`fn`'}.

keyword_nested_traits() ->
    _witness_type@0_0 =
        'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type::$patina_witness'(),
    _witness_type@1_0 =
        'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type#1::$patina_witness'(),
    _first@2_0 =
        'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type::get'(_witness_type@0_0,
                                                                                           1),
    _second@3_0 =
        'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type#1::get'(_witness_type@1_0,
                                                                                             1),
    _first@2_0 + _second@3_0.

keyword_nested_mods() ->
    _first@0_0 =
        'Elixir.Patina.BacktickIdent.keyword_nested_mods.Type':value(),
    _second@1_0 =
        'Elixir.Patina.BacktickIdent.keyword_nested_mods.Type#1':value(),
    _first@0_0 + _second@1_0.

unit_pattern() ->
    'Elixir.Patina.BacktickIdent.Unit':same('Elixir.Patina.BacktickIdent.Unit').

'Self'() -> {}.
