-file("tests/ui/src/generics/witness/blanket_impl.pt",
      4).

-module('Elixir.Patina.BlanketImpl').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BlanketImpl::Describe::shout'/2,
         wrap/1,
         describe_int/1,
         shout_str/1,
         describe_generic_int/1,
         describe_pair/2,
         shout_pair_generic/2,
         twice_str/1,
         twice_generic_pair/2]).

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

'BlanketImpl::Describe::shout'(_witness@0_0,
                               _self@1_0) ->
    <<((erlang:element(2,
                       _witness@0_0)):'BlanketImpl::Describe::describe'(_witness@0_0,
                                                                        _self@1_0))/binary,
      <<"!"/utf8>>/binary>>.

wrap(_s@0_0) ->
    <<<<<<"<"/utf8>>/binary, _s@0_0/binary>>/binary,
      <<">"/utf8>>/binary>>.

describe_generic({_witness@0_0}, _t@1_0) ->
    (erlang:element(2,
                    _witness@0_0)):'BlanketImpl::Describe::describe'(_witness@0_0,
                                                                     _t@1_0).

twice_generic({_witness@0_0}, _t@1_0) ->
    (erlang:element(2,
                    _witness@0_0)):'BlanketImpl::Nested::Twice::twice'(_witness@0_0,
                                                                       _t@1_0).

describe_int(_n@0_0) ->
    'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::describe'({'impl BlanketImpl::Describe for _',
                                                                         'Elixir.Patina.BlanketImpl.$impl',
                                                                         {'impl BlanketImpl::Show for int',
                                                                          'Elixir.Patina.BlanketImpl.$impl.Core.Int'}},
                                                                        _n@0_0).

shout_str(_s@0_0) ->
    'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::shout'({'impl BlanketImpl::Describe for _',
                                                                      'Elixir.Patina.BlanketImpl.$impl',
                                                                      {'impl BlanketImpl::Show for str',
                                                                       'Elixir.Patina.BlanketImpl.$impl.Core.Str'}},
                                                                     _s@0_0).

describe_generic_int(_n@0_0) ->
    describe_generic({{'impl BlanketImpl::Describe for _',
                       'Elixir.Patina.BlanketImpl.$impl',
                       {'impl BlanketImpl::Show for int',
                        'Elixir.Patina.BlanketImpl.$impl.Core.Int'}}},
                     _n@0_0).

describe_pair(_n@0_0, _s@1_0) ->
    'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::describe'({'impl BlanketImpl::Describe for (int, str)',
                                                                         'Elixir.Patina.BlanketImpl.$impl'},
                                                                        {_n@0_0,
                                                                         _s@1_0}).

shout_pair_generic(_n@0_0, _s@1_0) ->
    <<(describe_generic({{'impl BlanketImpl::Describe for (int, str)',
                          'Elixir.Patina.BlanketImpl.$impl'}},
                        {_n@0_0, _s@1_0}))/binary,
      ('Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::shout'({'impl BlanketImpl::Describe for (int, str)',
                                                                         'Elixir.Patina.BlanketImpl.$impl'},
                                                                        {_n@0_0,
                                                                         _s@1_0}))/binary>>.

twice_str(_s@0_0) ->
    'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Nested::Twice::twice'({'impl BlanketImpl::Nested::Twice for _',
                                                                           'Elixir.Patina.BlanketImpl.$impl',
                                                                           {'impl BlanketImpl::Describe for _',
                                                                            'Elixir.Patina.BlanketImpl.$impl',
                                                                            {'impl BlanketImpl::Show for str',
                                                                             'Elixir.Patina.BlanketImpl.$impl.Core.Str'}}},
                                                                          _s@0_0).

twice_generic_pair(_n@0_0, _s@1_0) ->
    twice_generic({{'impl BlanketImpl::Nested::Twice for _',
                    'Elixir.Patina.BlanketImpl.$impl',
                    {'impl BlanketImpl::Describe for (int, str)',
                     'Elixir.Patina.BlanketImpl.$impl'}}},
                  {_n@0_0, _s@1_0}).
