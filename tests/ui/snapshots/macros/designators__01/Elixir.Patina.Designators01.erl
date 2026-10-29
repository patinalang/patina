-file("tests/ui/src/macros/designators__01.pt", 4).

-module('Elixir.Patina.Designators01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

foo() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [<<"foo"/utf8>>]),
                                        [<<"You called "/utf8>>,
                                         _element@0_0,
                                         <<"()"/utf8>>]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {}.

bar() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [<<"bar"/utf8>>]),
                                        [<<"You called "/utf8>>,
                                         _element@0_0,
                                         <<"()"/utf8>>]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    {}.

main() ->
    foo(),
    bar(),
    io:put_chars(begin
                     _element@2_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [<<"1 + 1"/utf8>>]),
                                        _element@1_0 = io_lib:format("~ktp",
                                                                     [1 + 1]),
                                        [_element@0_0,
                                         <<" = "/utf8>>,
                                         _element@1_0]
                                    end,
                     [_element@2_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@6_0 = begin
                                        _element@3_0 = io_lib:format("~ktp",
                                                                     [<<"{ let x = 1; x * x + 2 * x - 1 }"/utf8>>]),
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [begin
                                                                          _x@4_0 =
                                                                              1,
                                                                          _x@4_0
                                                                              *
                                                                              _x@4_0
                                                                              +
                                                                              2
                                                                                  *
                                                                                  _x@4_0
                                                                              -
                                                                              1
                                                                      end]),
                                        [_element@3_0,
                                         <<" = "/utf8>>,
                                         _element@5_0]
                                    end,
                     [_element@6_0]
                 end),
    io:nl(),
    {}.
