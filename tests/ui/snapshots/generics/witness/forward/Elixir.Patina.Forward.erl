-file("tests/ui/src/generics/witness/forward.pt", 1).

-module('Elixir.Patina.Forward').

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

b(_T@0_0, _x@1_0) ->
    (erlang:element(1,
                    _T@0_0)):'Forward::Tr::method'(_T@0_0, _x@1_0).

a(_T@0_0, _x@1_0) -> b(_T@0_0, _x@1_0).

main() ->
    a({'Elixir.Patina.Forward.S', {}},
      'Elixir.Patina.Forward.S').
