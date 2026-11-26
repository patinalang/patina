-file("tests/ui/src/error/option_unwrap__01.pt", 6).

-module('Elixir.Patina.OptionUnwrap01').

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

give_adult(_drink@0_0) ->
    case _drink@0_0 of
        _value@1_0 = <<"lemonade"/utf8>>
            when erlang:is_binary(_value@1_0) ->
            io:put_chars([[<<"Yuck! Too sugary."/utf8>>]]),
            io:nl(),
            {};
        _value@3_0 = _inner@2_0
            when erlang:is_binary(_value@3_0) ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@4_0 = _inner@2_0,
                                                [_element@4_0,
                                                 <<"? How nice."/utf8>>]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            {};
        nil ->
            io:put_chars([[<<"No drink? Oh well."/utf8>>]]),
            io:nl(),
            {}
    end.

drink(_drink@0_0) ->
    _inside@4_0 = begin
                      _self@1_0 = _drink@0_0,
                      case _self@1_0 of
                          _value@3_0 = _v@2_0
                              when erlang:is_binary(_value@3_0) ->
                              _v@2_0;
                          nil ->
                              erlang:error('Elixir.Patina.Core.Panic':new(<<"called `Option::unwrap()` on a `None` "
                                                                            "value"/utf8>>),
                                           none,
                                           [{error_info,
                                             #{module =>
                                                   'Elixir.Patina.Core.Panic'}}])
                      end
                  end,
    case _inside@4_0 =:= <<"lemonade"/utf8>> of
        true ->
            erlang:error('Elixir.Patina.Core.Panic':new(<<"AAAaaaaa!!!!"/utf8>>),
                         none,
                         [{error_info,
                           #{module => 'Elixir.Patina.Core.Panic'}}]);
        _ -> {}
    end,
    io:put_chars(begin
                     _element@6_0 = begin
                                        _element@5_0 = _inside@4_0,
                                        [<<"I love "/utf8>>,
                                         _element@5_0,
                                         <<"s!!!!!"/utf8>>]
                                    end,
                     [_element@6_0]
                 end),
    io:nl(),
    {}.

main() ->
    _water@0_0 = <<"water"/utf8>>,
    _lemonade@1_0 = <<"lemonade"/utf8>>,
    _void@2_0 = nil,
    give_adult(_water@0_0),
    give_adult(_lemonade@1_0),
    give_adult(_void@2_0),
    _coffee@3_0 = <<"coffee"/utf8>>,
    _nothing@4_0 = nil,
    drink(_coffee@3_0),
    drink(_nothing@4_0).
