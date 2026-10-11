-file("tests/ui/src/trait/blanket_impl.pt", 36).

-module('Elixir.Patina.BlanketImpl.$impl.Core.Tuple').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BlanketImpl::Describe::describe'/2,
         'BlanketImpl::Describe::shout'/2]).

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

'BlanketImpl::Describe::describe'(_witness@0_0 =
                                      {'impl BlanketImpl::Describe for (int, str)',
                                       _},
                                  _self@1_0) ->
    <<<<('Elixir.Patina.BlanketImpl.$impl.Core.Int':'BlanketImpl::Show::show'({'impl BlanketImpl::Show for int',
                                                                               'Elixir.Patina.BlanketImpl.$impl.Core.Int'},
                                                                              erlang:element(1,
                                                                                             _self@1_0)))/binary,
        " and "/utf8>>/binary,
      ('Elixir.Patina.BlanketImpl.$impl.Core.Str':'BlanketImpl::Show::show'({'impl BlanketImpl::Show for str',
                                                                             'Elixir.Patina.BlanketImpl.$impl.Core.Str'},
                                                                            erlang:element(2,
                                                                                           _self@1_0)))/binary>>.

'BlanketImpl::Describe::shout'(_witness@0_0 =
                                   {'impl BlanketImpl::Describe for (int, str)',
                                    _},
                               _arg@1_0) ->
    'Elixir.Patina.BlanketImpl':'BlanketImpl::Describe::shout'(_witness@0_0,
                                                               _arg@1_0).
