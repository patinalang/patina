-file("tests/ui/src/trait/blanket_impl.pt", 1).

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

'BlanketImpl::Describe::shout'(_witness_Describe@0_0,
                               _self@1_0) ->
    _vtable_Describe@2_0 = erlang:element(1,
                                          _witness_Describe@0_0),
    _fn_Describe_describe@3_0 = erlang:element(1,
                                               _vtable_Describe@2_0),
    begin
        <<(_fn_Describe_describe@3_0(_witness_Describe@0_0,
                                     _self@1_0))/binary,
          "!"/utf8>>
    end.

wrap(_s@0_0) ->
    <<<<"<"/utf8, _s@0_0/binary>>/binary, ">"/utf8>>.

describe_generic(_witnesses_where_describe_generic@0_0,
                 _t@1_0) ->
    _witness_Describe@2_0 = erlang:element(1,
                                           _witnesses_where_describe_generic@0_0),
    _vtable_Describe@3_0 = erlang:element(1,
                                          _witness_Describe@2_0),
    _fn_Describe_describe@4_0 = erlang:element(1,
                                               _vtable_Describe@3_0),
    begin
        _fn_Describe_describe@4_0(_witness_Describe@2_0, _t@1_0)
    end.

twice_generic(_witnesses_where_twice_generic@0_0,
              _t@1_0) ->
    _witness_Twice@2_0 = erlang:element(1,
                                        _witnesses_where_twice_generic@0_0),
    _vtable_Twice@3_0 = erlang:element(1,
                                       _witness_Twice@2_0),
    _fn_Twice_twice@4_0 = erlang:element(1,
                                         _vtable_Twice@3_0),
    begin
        _fn_Twice_twice@4_0(_witness_Twice@2_0, _t@1_0)
    end.

describe_int(_n@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Int':'BlanketImpl::Show::$patina_witness'(),
    _witnesses_where_Describe@2_0 = {_witness_Show@1_0},
    _witness_Describe@3_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::$patina_witness'(_witnesses_where_Describe@2_0),
    begin
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::describe'(_witness_Describe@3_0,
                                                                            _n@0_0)
    end.

shout_str(_s@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Str':'BlanketImpl::Show::$patina_witness'(),
    _witnesses_where_Describe@2_0 = {_witness_Show@1_0},
    _witness_Describe@3_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::$patina_witness'(_witnesses_where_Describe@2_0),
    begin
        'BlanketImpl::Describe::shout'(_witness_Describe@3_0,
                                       _s@0_0)
    end.

describe_generic_int(_n@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Int':'BlanketImpl::Show::$patina_witness'(),
    _witnesses_where_Describe@2_0 = {_witness_Show@1_0},
    _witness_Describe@3_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::$patina_witness'(_witnesses_where_Describe@2_0),
    _witnesses_where_describe_generic@4_0 =
        {_witness_Describe@3_0},
    begin
        describe_generic(_witnesses_where_describe_generic@4_0,
                         _n@0_0)
    end.

describe_pair(_n@0_0, _s@1_0) ->
    _witness_Describe@2_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Tuple':'BlanketImpl::Describe::$patina_witness<(int, str)>'(),
    begin
        'Elixir.Patina.BlanketImpl.$impl.Core.Tuple':'BlanketImpl::Describe::describe<(int, str)>'(_witness_Describe@2_0,
                                                                                                   {_n@0_0,
                                                                                                    _s@1_0})
    end.

shout_pair_generic(_n@0_0, _s@1_0) ->
    _witness_Describe@2_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Tuple':'BlanketImpl::Describe::$patina_witness<(int, str)>'(),
    _witnesses_where_describe_generic@3_0 =
        {_witness_Describe@2_0},
    begin
        <<(describe_generic(_witnesses_where_describe_generic@3_0,
                            {_n@0_0, _s@1_0}))/binary,
          ('BlanketImpl::Describe::shout'(_witness_Describe@2_0,
                                          {_n@0_0, _s@1_0}))/binary>>
    end.

twice_str(_s@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Str':'BlanketImpl::Show::$patina_witness'(),
    _witnesses_where_Describe@2_0 = {_witness_Show@1_0},
    _witness_Describe@3_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::$patina_witness'(_witnesses_where_Describe@2_0),
    _witnesses_where_Twice@4_0 = {_witness_Describe@3_0},
    _witness_Twice@5_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Nested::Twice::$patina_witness'(_witnesses_where_Twice@4_0),
    begin
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Nested::Twice::twice'(_witness_Twice@5_0,
                                                                              _s@0_0)
    end.

twice_generic_pair(_n@0_0, _s@1_0) ->
    _witness_Describe@2_0 =
        'Elixir.Patina.BlanketImpl.$impl.Core.Tuple':'BlanketImpl::Describe::$patina_witness<(int, str)>'(),
    _witnesses_where_Twice@3_0 = {_witness_Describe@2_0},
    _witness_Twice@4_0 =
        'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Nested::Twice::$patina_witness'(_witnesses_where_Twice@3_0),
    _witnesses_where_twice_generic@5_0 =
        {_witness_Twice@4_0},
    begin
        twice_generic(_witnesses_where_twice_generic@5_0,
                      {_n@0_0, _s@1_0})
    end.
