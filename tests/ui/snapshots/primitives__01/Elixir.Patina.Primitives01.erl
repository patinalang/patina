-file("tests/ui/src/primitives__01.pt", 4).

-module('Elixir.Patina.Primitives01').

-compile([no_auto_import]).

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
    _logical@0_0 = true,
    _a_float@1_0 = 1.0,
    _an_atom@2_0 = hello,
    _inferred_type@3_1 = add_floats(1.2e+1, 3.14),
    _mutable@4_0 = 12,
    _mutable@4_1 = 21,
    _mutable@5_0 = true,
    _my_list@6_0 = [1, 2, 3, 4, 5],
    _my_tuple@7_0 = {5, true, -5.04},
    _my_map@8_0 = #{3.14 => <<"pi">>, 0.0 => <<"zero">>},
    _add_ref@9_0 = fun add_floats/2,
    _add_fn@10_0 = _add_ref@9_0.

add_floats(_a@0_0, _b@1_0) -> _a@0_0 + _b@1_0.
