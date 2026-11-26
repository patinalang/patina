-file("tests/ui/src/error/option_unwrap/question_mar"
      "k__02.pt",
      4).

-module('Elixir.Patina.QuestionMark02').

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
    _p@6_0 = begin
                 _job@5_0 = begin
                                _0@3_0 = begin
                                             _phone_number@2_0 = begin
                                                                     _0@1_0 =
                                                                         begin
                                                                             _area_code@0_0 =
                                                                                 61,
                                                                             'Elixir.Patina.QuestionMark02.PhoneNumber':'$patina_constructor__PhoneNumber'(_area_code@0_0,
                                                                                                                                                           439222222)
                                                                         end,
                                                                     _0@1_0
                                                                 end,
                                             _phone_number@2_0
                                         end,
                                case _0@3_0 of
                                    _value@4_0 when _value@4_0 =:= nil ->
                                        {some, _value@4_0};
                                    _value@4_0 -> _value@4_0
                                end
                            end,
                 _job@5_0
             end,
    {_left_val@9_0, _right_val@10_0} = begin
                                           _element@7_0 =
                                               'Elixir.Patina.QuestionMark02.Person':work_phone_area_code(_p@6_0),
                                           _element@8_0 = 61,
                                           {_element@7_0, _element@8_0}
                                       end,
    case not
             begin
                 _self@11_0 = _left_val@9_0,
                 _other@12_0 = _right_val@10_0,
                 _self@11_0 =:= _other@12_0
             end
        of
        true ->
            erlang:error('Elixir.Patina.Core.Panic':new(unicode:characters_to_binary(begin
                                                                                         _element@13_0 =
                                                                                             io_lib:format("~ktp",
                                                                                                           [_left_val@9_0]),
                                                                                         _element@14_0 =
                                                                                             io_lib:format("~ktp",
                                                                                                           [_right_val@10_0]),
                                                                                         [<<"assertion `left == right` failed"/utf8>>,
                                                                                          <<""/utf8>>,
                                                                                          <<"\n  left: "/utf8>>,
                                                                                          _element@13_0,
                                                                                          <<"\n right: "/utf8>>,
                                                                                          _element@14_0]
                                                                                     end)),
                         none,
                         [{error_info,
                           #{module => 'Elixir.Patina.Core.Panic'}}]);
        _ -> {}
    end.
