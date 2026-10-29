-file("tests/ui/src/unsafe__02.pt", 4).

-module('Elixir.Patina.Unsafe02').

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
    _safe_description@3_0 = case _v@0_0 of
                                nil -> <<"the atom nil"/utf8>>;
                                {some, _val@1_0} ->
                                    <<"some value (tagged?)"/utf8>>;
                                _val@2_0 -> <<"some value (untagged?)"/utf8>>
                            end,
    _unsafe_description@4_0 = case _v@0_0 of
                                  nil -> <<"an empty string option"/utf8>>;
                                  nil -> <<"an empty integer option"/utf8>>;
                                  nil -> <<"the atom nil"/utf8>>;
                                  _ -> <<"something else"/utf8>>
                              end,
    io:put_chars(begin
                     _element@7_0 = begin
                                        _element@5_0 = _safe_description@3_0,
                                        _element@6_0 = _unsafe_description@4_0,
                                        [_element@5_0,
                                         <<", "/utf8>>,
                                         _element@6_0]
                                    end,
                     [_element@7_0]
                 end),
    io:nl(),
    {}.
