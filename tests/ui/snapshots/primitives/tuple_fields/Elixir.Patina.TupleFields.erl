-file("tests/ui/src/primitives/tuple_fields.pt", 1).

-module('Elixir.Patina.TupleFields').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         first_of_three/1,
         last_of_three/1,
         nested/1,
         generic_second/2,
         single/1,
         of_call/1]).

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

second(_pair@0_0) -> erlang:element(2, _pair@0_0).

first_of_three(_t@0_0) -> erlang:element(1, _t@0_0).

last_of_three(_t@0_0) -> erlang:element(3, _t@0_0).

nested(_t@0_0) ->
    erlang:element(2, erlang:element(2, _t@0_0)).

generic_second(_n@0_0, _s@1_0) ->
    second({_n@0_0, _s@1_0}).

single(_t@0_0) -> erlang:element(1, _t@0_0).

of_call(_n@0_0) ->
    erlang:element(1,
                   second(begin
                              _element@3_0 = begin
                                                 _element@1_0 = _n@0_0 + 1,
                                                 _element@2_0 = _n@0_0 + 2,
                                                 {_element@1_0, _element@2_0}
                                             end,
                              {_n@0_0, _element@3_0}
                          end)).
