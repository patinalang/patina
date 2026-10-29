-file("tests/ui/src/trait/blanket_impl.pt", 36).

-module('Elixir.Patina.BlanketImpl.$impl.Core.Tuple').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BlanketImpl::Describe::describe<(int, str)>'/2,
         'BlanketImpl::Describe::$patina_witness<(int, str)>'/0]).

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

'BlanketImpl::Describe::describe<(int, str)>'(_witness_Describe@0_0,
                                              _self@1_0) ->
    _witness_Show@2_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Int':'BlanketImpl::Show::$patina_witness'(),
    _witness_Show@3_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Str':'BlanketImpl::Show::$patina_witness'(),
    <<<<('Elixir.Patina.BlanketImpl.$impl.Core.Int':'BlanketImpl::Show::show'(_witness_Show@2_0,
                                                                              erlang:element(1,
                                                                                             _self@1_0)))/binary,
        " and "/utf8>>/binary,
      ('Elixir.Patina.BlanketImpl.$impl.Core.Str':'BlanketImpl::Show::show'(_witness_Show@3_0,
                                                                            erlang:element(2,
                                                                                           _self@1_0)))/binary>>.

'BlanketImpl::Describe::$patina_witness<(int, str)>'() ->
    {{fun 'Elixir.Patina.BlanketImpl.$impl.Core.Tuple':'BlanketImpl::Describe::describe<(int, str)>'/2,
      fun 'Elixir.Patina.BlanketImpl':'BlanketImpl::Describe::shout'/2},
     {},
     {}}.
