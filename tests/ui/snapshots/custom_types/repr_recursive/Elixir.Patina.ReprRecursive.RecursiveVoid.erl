-file("tests/ui/src/custom_types/repr_recursive.pt",
      162).

-module('Elixir.Patina.ReprRecursive.RecursiveVoid').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['RecursiveVoid']).

-export(['__info__'/1,
         '$patina_constructor__RecursiveVoid'/2]).

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

-record #'RecursiveVoid'{'0', '1'}.

'$patina_constructor__RecursiveVoid'(_0@0_0, _1@1_0) ->
    #'RecursiveVoid'{'0' = _0@0_0, '1' = _1@1_0}.
