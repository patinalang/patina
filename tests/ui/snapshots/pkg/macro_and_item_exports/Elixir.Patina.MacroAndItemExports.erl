-file("tests/ui/src/pkg/macro_and_item_exports.pt", 1).

-module('Elixir.Patina.MacroAndItemExports').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, items/0, macros/0]).

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

derived_value() -> 42.

items() ->
    _witness_Describe@0_0 =
        'Elixir.Patina.MacrosAndItems.Plain':'macros_and_items::Describe::$patina_witness'(),
    _n@1_0 = 5,
    io:put_chars(begin
                     _element@5_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.MacrosAndItems.Plain':'macros_and_items::Describe::describe'(_witness_Describe@0_0,
                                                                                                                                                  _n@1_0)]),
                                        _element@3_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.MacrosAndItems':plain()]),
                                        _element@4_0 = io_lib:format("~ktp",
                                                                     [derived_value()]),
                                        [_element@2_0,
                                         <<" "/utf8>>,
                                         _element@3_0,
                                         <<" "/utf8>>,
                                         _element@4_0]
                                    end,
                     [_element@5_0]
                 end),
    io:nl(),
    {}.

macros() ->
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [5 + 5]),
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [7]),
                                        [_element@0_0,
                                         <<" "/utf8>>,
                                         _element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    {}.
