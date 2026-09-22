-file("tests/ui/src/flow_control/match/unsafe_match_"
      "_01.pt",
      4).

-module('Elixir.Patina.UnsafeMatch01').

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
    _v@0_0 = nil,
    _safe_description@4_0 = case _v@0_0 of
                                nil -> <<"the atom nil">>;
                                {some, _val@1_0} -> <<"some value (tagged?)">>;
                                _value@2_0 when erlang:is_integer(_value@2_0) ->
                                    <<"some integer value (untagged?)">>;
                                _val@3_0 -> <<"some value (untagged?)">>
                            end,
    _unsafe_description@5_0 = begin
                                  case _v@0_0 of
                                      nil -> <<"an empty string option">>;
                                      nil -> <<"an empty integer option">>;
                                      nil -> <<"the atom nil">>;
                                      _ -> <<"something else">>
                                  end
                              end,
    begin
        io:put_chars([[_safe_description@4_0,
                       <<", ">>,
                       _unsafe_description@5_0]]),
        io:nl(),
        {}
    end.
