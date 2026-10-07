-file("tests/ui/src/trait/blanket_impl.pt", 11).

-module('Elixir.Patina.BlanketImpl.$impl.Core.Str').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, 'BlanketImpl::Show::show'/2]).

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

'BlanketImpl::Show::show'(_witness@0_0 =
                              {'impl BlanketImpl::Show for str', _},
                          _self@1_0) ->
    _self@1_0.
