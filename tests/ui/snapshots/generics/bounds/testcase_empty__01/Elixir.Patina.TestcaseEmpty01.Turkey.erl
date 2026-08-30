-file("tests/ui/src/generics/bounds/testcase_empty__"
      "01.pt",
      6).

-module('Elixir.Patina.TestcaseEmpty01.Turkey').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1, '__struct__'/0, '__struct__'/1]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) -> [];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' =>
          'Elixir.Patina.TestcaseEmpty01.Turkey'}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).
