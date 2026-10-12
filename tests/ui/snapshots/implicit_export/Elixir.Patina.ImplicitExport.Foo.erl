-file("tests/ui/src/implicit_export.pt", 1).

-module('Elixir.Patina.ImplicitExport.Foo').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         foo/0,
         'ImplicitExport::FooTrait::bar'/1,
         'ImplicitExport::FooTrait::$patina_witness'/0]).

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

foo() -> priv_foo().

priv_foo() -> {}.

'ImplicitExport::FooTrait::bar'(_witness_self@0_0 = {_,
                                                     {},
                                                     {}}) ->
    priv_foo().

'ImplicitExport::FooTrait::$patina_witness'() ->
    {{fun 'Elixir.Patina.ImplicitExport.Foo':'ImplicitExport::FooTrait::bar'/1},
     {},
     {}}.
