-file("tests/ui/src/trait/disambiguating__01.pt", 4).

-module('Elixir.Patina.Disambiguating01').

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
    _witness_UsernameWidget@0_0 =
        'Elixir.Patina.Disambiguating01.Form':'disambiguating__01::UsernameWidget::$patina_witness'(),
    _witness_AgeWidget@1_0 =
        'Elixir.Patina.Disambiguating01.Form':'disambiguating__01::AgeWidget::$patina_witness'(),
    _form@2_0 =
        'Elixir.Patina.Disambiguating01.Form':'$patina_constructor__Form'(<<"rustacean"/utf8>>,
                                                                          28),
    _username@3_0 =
        'Elixir.Patina.Disambiguating01.Form':'disambiguating__01::UsernameWidget::get'(_witness_UsernameWidget@0_0,
                                                                                        _form@2_0),
    case {<<"rustacean"/utf8>>, _username@3_0} of
        {_left_val@4_0, _right_val@5_0} ->
            case not (_left_val@4_0 =:= _right_val@5_0) of
                true ->
                    erlang:error('Elixir.Patina.Core.Panic':new(unicode:characters_to_binary(begin
                                                                                                 _element@6_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_left_val@4_0]),
                                                                                                 _element@7_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_right_val@5_0]),
                                                                                                 [<<"assertion `left == right` failed"/utf8>>,
                                                                                                  <<""/utf8>>,
                                                                                                  <<"\n  left: "/utf8>>,
                                                                                                  _element@6_0,
                                                                                                  <<"\n right: "/utf8>>,
                                                                                                  _element@7_0]
                                                                                             end)),
                                 none,
                                 [{error_info,
                                   #{module => 'Elixir.Patina.Core.Panic'}}]);
                _ -> {}
            end
    end,
    _age@8_0 =
        'Elixir.Patina.Disambiguating01.Form':'disambiguating__01::AgeWidget::get'(_witness_AgeWidget@1_0,
                                                                                   _form@2_0),
    case {28, _age@8_0} of
        {_left_val@9_0, _right_val@10_0} ->
            case not (_left_val@9_0 =:= _right_val@10_0) of
                true ->
                    erlang:error('Elixir.Patina.Core.Panic':new(unicode:characters_to_binary(begin
                                                                                                 _element@11_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_left_val@9_0]),
                                                                                                 _element@12_0 =
                                                                                                     io_lib:format("~ktp",
                                                                                                                   [_right_val@10_0]),
                                                                                                 [<<"assertion `left == right` failed"/utf8>>,
                                                                                                  <<""/utf8>>,
                                                                                                  <<"\n  left: "/utf8>>,
                                                                                                  _element@11_0,
                                                                                                  <<"\n right: "/utf8>>,
                                                                                                  _element@12_0]
                                                                                             end)),
                                 none,
                                 [{error_info,
                                   #{module => 'Elixir.Patina.Core.Panic'}}]);
                _ -> {}
            end
    end.
