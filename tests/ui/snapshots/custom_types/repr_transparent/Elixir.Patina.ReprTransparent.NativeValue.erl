-file("tests/ui/src/custom_types/repr_transparent.pt",
      11).

-module('Elixir.Patina.ReprTransparent.NativeValue').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['NativeValue']).

-export(['__info__'/1,
         '$patina_constructor__NativeValue'/1]).

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

-record #'NativeValue'{value}.

'$patina_constructor__NativeValue'(_value@0_0) ->
    #'NativeValue'{value = _value@0_0}.
