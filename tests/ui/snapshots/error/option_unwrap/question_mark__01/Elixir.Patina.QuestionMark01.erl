-file("tests/ui/src/error/option_unwrap/question_mar"
      "k__01.pt",
      4).

-module('Elixir.Patina.QuestionMark01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1]).

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

next_birthday(_current_age@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@11_0} ->
            _jump_value@11_0;
        _ ->
            _next_age@8_0 = case begin
                                     _self@1_0 = _current_age@0_0,
                                     case _self@1_0 of
                                         _value@3_0 = _value@2_0
                                             when
                                               erlang:is_integer(_value@3_0) ->
                                             {continue, _value@2_0};
                                         nil -> {break, nil}
                                     end
                                 end
                                of
                                {break, _residual@4_0} ->
                                    _jump_value@6_0 = begin
                                                          __residual@5_0 =
                                                              _residual@4_0,
                                                          nil
                                                      end,
                                    '$patina_marker':jump('$patina_label_0',
                                                          _jump_value@6_0),
                                    [];
                                {continue, _val@7_0} -> _val@7_0
                            end
                                + 1,
            _0@10_0 = unicode:characters_to_binary(begin
                                                       _element@9_0 =
                                                           io_lib:format("~ktp",
                                                                         [_next_age@8_0]),
                                                       [<<"Next year I will be "/utf8>>,
                                                        _element@9_0]
                                                   end),
            _0@10_0
    end.
