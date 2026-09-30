-file("tests/ui/src/generics/inherent_tags.pt", 6).

-module('Elixir.Patina.InherentTags.GenVal').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'value<int>'/1,
         'value<str>'/1,
         f/2,
         'f<int>'/2]).

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

'value<int>'(_self@0_0) -> _self@0_0.

'value<str>'(_self@0_0) -> _self@0_0.

f({_witness@0_0}, _self@1_0) ->
    (erlang:element(2,
                    _witness@0_0)):'InherentTags::Show::show'(_witness@0_0,
                                                              _self@1_0).

'f<int>'(_self@0_0, _x@1_0) -> _self@0_0 + _x@1_0.
