-file("tests/ui/src/generics/witness/global_bound.pt",
      1).

-module('Elixir.Patina.GlobalBound').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, base/0, global/0]).

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

call(_witness_bounds@1_0 = {_witness_bound@0_0},
     _t@2_0) ->
    begin
        _witness@3_0 = erlang:element(1,
                                      erlang:element(2, _witness_bound@0_0)),
        (erlang:element(1,
                        erlang:element(1, _witness@3_0)))(_witness@3_0, _t@2_0)
    end.

only_global_bound() ->
    'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::derived'('Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::$patina_witness'(),
                                                                  'Elixir.Patina.GlobalBound.S').

base() ->
    call({'Elixir.Patina.GlobalBound.S':'GlobalBound::Derived::$patina_witness'()},
         'Elixir.Patina.GlobalBound.S').

global() -> only_global_bound().
