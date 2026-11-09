-file("tests/ui/src/pkg/foreign_inline.pt", 1).

-module('Elixir.Patina.ForeignInline').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         copy_calls_private_fn/0,
         inlines_foreign_impl_method/0,
         inlines_foreign_generic_with_foreign_impl/0,
         foreign_generic_calls_local_impl_statically/0]).

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

copy_calls_private_fn() ->
    _x@0_0 = 1,
    'Elixir.Patina.Inlines':private_fn(_x@0_0).

inlines_foreign_impl_method() ->
    _self@0_0 = 2,
    _self@0_0 + 1.

inlines_foreign_generic_with_foreign_impl() ->
    _x@0_0 = 2,
    begin _self@1_0 = _x@0_0, _self@1_0 + 1 end * 2.

foreign_generic_calls_local_impl_statically() ->
    _witness_Trait@0_0 =
        'Elixir.Patina.ForeignInline.LocalImpl':'inlines::Trait::$patina_witness'(),
    _x@1_0 = 5,
    'Elixir.Patina.ForeignInline.LocalImpl':'inlines::Trait::method'(_witness_Trait@0_0,
                                                                     _x@1_0)
        * 2.
