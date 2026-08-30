-file("tests/ui/src/types/numeric_casts.pt", 1).
-module('Elixir.Patina.NumericCasts').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         float_to_int/1,
         int_to_float/1,
         float_to_int_to_term/1,
         int_to_float_to_term/1,
         main/0]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
float_to_int(_value@0_0) ->
    trunc(_value@0_0).
int_to_float(_value@0_0) ->
    erlang:float(_value@0_0).
float_to_int_to_term(_value@0_0) ->
    trunc(_value@0_0).
int_to_float_to_term(_value@0_0) ->
    erlang:float(_value@0_0).
main() ->
    _ = float_to_int(3.14),
    _ = int_to_float(42),
    _ = float_to_int_to_term(3.14),
    _ = int_to_float_to_term(42).
