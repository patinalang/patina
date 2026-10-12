-file("tests/ui/src/local_calls.pt", 1).

-module('Elixir.Patina.LocalCalls').

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

get_age(_age@0_0) -> _age@0_0.

main() ->
    _age@0_0 = 'Elixir.Patina.LocalCalls.Years':new(25),
    _age_value@1_0 = get_age(_age@0_0),
    begin
        io:put_chars(begin
                         _element@6_0 = begin
                                            _element@2_0 = <<"Age "/utf8>>,
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [_age@0_0]),
                                            _element@4_0 = <<" is "/utf8>>,
                                            _element@5_0 = io_lib:format("~ktp",
                                                                         [_age_value@1_0]),
                                            [_element@2_0,
                                             _element@3_0,
                                             _element@4_0,
                                             _element@5_0]
                                        end,
                         [_element@6_0]
                     end),
        io:nl(),
        {}
    end.
