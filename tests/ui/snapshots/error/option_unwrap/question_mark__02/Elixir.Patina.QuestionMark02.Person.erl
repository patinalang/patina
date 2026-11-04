-file("tests/ui/src/error/option_unwrap/question_mar"
      "k__02.pt",
      4).

-module('Elixir.Patina.QuestionMark02.Person').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, work_phone_area_code/1]).

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

work_phone_area_code(_self@0_0) ->
    case [] of
        {'$patina_label_0', _jump_value@15_0} ->
            _jump_value@15_0;
        _ ->
            case begin
                     _self@8_0 = case begin
                                          _self@1_0 = _self@0_0,
                                          case _self@1_0 of
                                              {some, _value@2_0} ->
                                                  {continue, _value@2_0};
                                              _value@3_0 = _value@2_0
                                                  when not
                                                           (_value@3_0 =:=
                                                                nil) ->
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
                                 end,
                     case _self@8_0 of
                         _value@10_0 = _value@9_0
                             when erlang:is_record(_value@10_0,
                                                   'Elixir.Patina.QuestionMark02.PhoneNumber',
                                                   'PhoneNumber') ->
                             {continue, _value@9_0};
                         nil -> {break, nil}
                     end
                 end
                of
                {break, _residual@11_0} ->
                    _jump_value@13_0 = begin
                                           __residual@12_0 = _residual@11_0, nil
                                       end,
                    '$patina_marker':jump('$patina_label_0',
                                          _jump_value@13_0),
                    [];
                {continue, _val@14_0} -> _val@14_0
            end#'Elixir.Patina.QuestionMark02.PhoneNumber':'PhoneNumber'.area_code
    end.
