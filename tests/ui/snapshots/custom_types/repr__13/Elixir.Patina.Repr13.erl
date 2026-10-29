-file("tests/ui/src/custom_types/repr__13.pt", 4).

-module('Elixir.Patina.Repr13').

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

main() ->
    io:put_chars(begin
                     _element@1_0 = begin
                                        _element@0_0 = io_lib:format("~ktp",
                                                                     [42]),
                                        [_element@0_0]
                                    end,
                     [_element@1_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [nil]),
                                        [_element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@6_0 = begin
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [case hi of
                                                                          _value@4_0
                                                                              when
                                                                                _value@4_0
                                                                                    =:=
                                                                                    nil ->
                                                                              {some,
                                                                               _value@4_0};
                                                                          _value@4_0 ->
                                                                              _value@4_0
                                                                      end]),
                                        [_element@5_0]
                                    end,
                     [_element@6_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@9_0 = begin
                                        _element@8_0 = io_lib:format("~ktp",
                                                                     [case nil
                                                                          of
                                                                          _value@7_0
                                                                              when
                                                                                _value@7_0
                                                                                    =:=
                                                                                    nil ->
                                                                              {some,
                                                                               _value@7_0};
                                                                          _value@7_0 ->
                                                                              _value@7_0
                                                                      end]),
                                        [_element@8_0]
                                    end,
                     [_element@9_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@13_0 = begin
                                         _element@12_0 = io_lib:format("~ktp",
                                                                       [begin
                                                                            _0@10_0 =
                                                                                42,
                                                                            case
                                                                                _0@10_0
                                                                                of
                                                                                _value@11_0
                                                                                    when
                                                                                      _value@11_0
                                                                                          =:=
                                                                                          nil ->
                                                                                    {some,
                                                                                     _value@11_0};
                                                                                _value@11_0 ->
                                                                                    _value@11_0
                                                                            end
                                                                        end]),
                                         [_element@12_0]
                                     end,
                     [_element@13_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@16_0 = begin
                                         _element@15_0 = io_lib:format("~ktp",
                                                                       [case nil
                                                                            of
                                                                            _value@14_0
                                                                                when
                                                                                  _value@14_0
                                                                                      =:=
                                                                                      nil ->
                                                                                {some,
                                                                                 _value@14_0};
                                                                            _value@14_0 ->
                                                                                _value@14_0
                                                                        end]),
                                         [_element@15_0]
                                     end,
                     [_element@16_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@18_0 = begin
                                         _element@17_0 = io_lib:format("~ktp",
                                                                       [nil]),
                                         [_element@17_0]
                                     end,
                     [_element@18_0]
                 end),
    io:nl(),
    {}.
