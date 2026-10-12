-file("tests/ui/src/custom_types/repr__09.pt", 4).

-module('Elixir.Patina.Repr09').

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
    begin
        io:put_chars(begin
                         _element@2_0 = begin
                                            _element@1_0 = io_lib:format("~ktp",
                                                                         [case
                                                                              <<"hello"/utf8>>
                                                                              of
                                                                              _value@0_0
                                                                                  when
                                                                                    erlang:is_binary(_value@0_0) ->
                                                                                  {othertag,
                                                                                   _value@0_0};
                                                                              _value@0_0 ->
                                                                                  _value@0_0
                                                                          end]),
                                            [_element@1_0]
                                        end,
                         [_element@2_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@4_0 = begin
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [42]),
                                            [_element@3_0]
                                        end,
                         [_element@4_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@8_0 = begin
                                            _element@7_0 = io_lib:format("~ktp",
                                                                         [begin
                                                                              _0@5_0 =
                                                                                  hi,
                                                                              case
                                                                                  _0@5_0
                                                                                  of
                                                                                  _value@6_0
                                                                                      when
                                                                                        erlang:is_binary(_value@6_0) ->
                                                                                      {othertag,
                                                                                       _value@6_0};
                                                                                  _value@6_0
                                                                                      when
                                                                                        (erlang:is_tuple(_value@6_0)
                                                                                             andalso
                                                                                             erlang:tuple_size(_value@6_0)
                                                                                                 =:=
                                                                                                 2)
                                                                                            andalso
                                                                                            erlang:element(1,
                                                                                                           _value@6_0)
                                                                                                =:=
                                                                                                othertag ->
                                                                                      {othertag,
                                                                                       _value@6_0};
                                                                                  _value@6_0
                                                                                      when
                                                                                        (erlang:is_tuple(_value@6_0)
                                                                                             andalso
                                                                                             erlang:tuple_size(_value@6_0)
                                                                                                 =:=
                                                                                                 2)
                                                                                            andalso
                                                                                            erlang:element(1,
                                                                                                           _value@6_0)
                                                                                                =:=
                                                                                                mytag ->
                                                                                      {othertag,
                                                                                       _value@6_0};
                                                                                  _value@6_0 ->
                                                                                      _value@6_0
                                                                              end
                                                                          end]),
                                            [_element@7_0]
                                        end,
                         [_element@8_0]
                     end),
        io:nl(),
        {}
    end,
    begin
        io:put_chars(begin
                         _element@12_0 = begin
                                             _element@11_0 =
                                                 io_lib:format("~ktp",
                                                               [begin
                                                                    _0@9_0 =
                                                                        {othertag,
                                                                         1},
                                                                    case _0@9_0
                                                                        of
                                                                        _value@10_0
                                                                            when
                                                                              erlang:is_binary(_value@10_0) ->
                                                                            {othertag,
                                                                             _value@10_0};
                                                                        _value@10_0
                                                                            when
                                                                              (erlang:is_tuple(_value@10_0)
                                                                                   andalso
                                                                                   erlang:tuple_size(_value@10_0)
                                                                                       =:=
                                                                                       2)
                                                                                  andalso
                                                                                  erlang:element(1,
                                                                                                 _value@10_0)
                                                                                      =:=
                                                                                      othertag ->
                                                                            {othertag,
                                                                             _value@10_0};
                                                                        _value@10_0
                                                                            when
                                                                              (erlang:is_tuple(_value@10_0)
                                                                                   andalso
                                                                                   erlang:tuple_size(_value@10_0)
                                                                                       =:=
                                                                                       2)
                                                                                  andalso
                                                                                  erlang:element(1,
                                                                                                 _value@10_0)
                                                                                      =:=
                                                                                      mytag ->
                                                                            {othertag,
                                                                             _value@10_0};
                                                                        _value@10_0 ->
                                                                            _value@10_0
                                                                    end
                                                                end]),
                                             [_element@11_0]
                                         end,
                         [_element@12_0]
                     end),
        io:nl(),
        {}
    end.
