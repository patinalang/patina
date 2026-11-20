-file("tests/ui/src/flow_control/try_catch_after__02.pt",
      4).

-module('Elixir.Patina.TryCatchAfter02').

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
    _value@1_0 = try erlang:error({badmatch, 42}), 0 catch
                     error:{badmatch, _n@0_0}
                         when erlang:is_integer(_n@0_0) ->
                         _n@0_0
                 end,
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_value@1_0]),
                                        [_element@2_0]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    {}.
