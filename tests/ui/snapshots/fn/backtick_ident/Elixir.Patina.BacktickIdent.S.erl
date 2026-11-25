-file("tests/ui/src/fn/backtick_ident.pt", 43).

-module('Elixir.Patina.BacktickIdent.S').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'backtick_ident::type::Shape::fn'/2,
         'backtick_ident::type::Shape::$patina_witness'/0,
         'valid?'/1,
         check_via_self/1,
         'backtick_ident::keyword_nested_traits::type::get'/2,
         'backtick_ident::keyword_nested_traits::type::$patina_witness'/0,
         'backtick_ident::keyword_nested_traits::type#1::get'/2,
         'backtick_ident::keyword_nested_traits::type#1::$patina_witness'/0]).

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

'backtick_ident::type::Shape::fn'(_witness_Shape@0_0,
                                  _self@1_0) ->
    _self@1_0.

'backtick_ident::type::Shape::$patina_witness'() ->
    {{fun 'Elixir.Patina.BacktickIdent.S':'backtick_ident::type::Shape::fn'/2},
     {},
     {}}.

'valid?'(_self@0_0) -> _self@0_0 > 0.

check_via_self(_s@0_0) -> 'valid?'(_s@0_0).

'backtick_ident::keyword_nested_traits::type::get'(_witness_type@0_0,
                                                   _self@1_0) ->
    _self@1_0.

'backtick_ident::keyword_nested_traits::type::$patina_witness'() ->
    {{fun 'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type::get'/2},
     {},
     {}}.

'backtick_ident::keyword_nested_traits::type#1::get'(_witness_type@0_0,
                                                     _self@1_0) ->
    _self@1_0 + 1.

'backtick_ident::keyword_nested_traits::type#1::$patina_witness'() ->
    {{fun 'Elixir.Patina.BacktickIdent.S':'backtick_ident::keyword_nested_traits::type#1::get'/2},
     {},
     {}}.
