-file("tests/ui/src/generics/witness/assoc_bound.pt",
      4).

-module('Elixir.Patina.AssocBound').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         counter_first/1,
         wrapper_plain/0,
         wrapper_member/0]).

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

first(_witnesses_where_first@0_0, _i@1_0) ->
    _witness_Iter@2_0 = erlang:element(1,
                                       _witnesses_where_first@0_0),
    _vtable_Iter@3_0 = erlang:element(1, _witness_Iter@2_0),
    _fn_Iter_next@4_0 = erlang:element(2, _vtable_Iter@3_0),
    _type_Iter_Item@5_0 = erlang:element(1,
                                         _vtable_Iter@3_0),
    _where_Iter@6_0 =
        _type_Iter_Item@5_0(_witness_Iter@2_0),
    _witness_Show@7_0 = erlang:element(1, _where_Iter@6_0),
    _vtable_Show@8_0 = erlang:element(1, _witness_Show@7_0),
    _fn_Show_show@9_0 = erlang:element(1, _vtable_Show@8_0),
    begin
        _fn_Show_show@9_0(_witness_Show@7_0,
                          _fn_Iter_next@4_0(_witness_Iter@2_0, _i@1_0))
    end.

counter_first(_n@0_0) ->
    _witness_Iter@1_0 =
        'Elixir.Patina.AssocBound.Counter':'AssocBound::Iter::$patina_witness'(),
    _witnesses_where_first@2_0 = {_witness_Iter@1_0},
    begin first(_witnesses_where_first@2_0, _n@0_0) end.

show_plain(_witnesses_where_show_plain@0_0, _f@1_0) ->
    _witness_Family@2_0 = erlang:element(1,
                                         _witnesses_where_show_plain@0_0),
    _vtable_Family@3_0 = erlang:element(1,
                                        _witness_Family@2_0),
    _fn_Family_plain@4_0 = erlang:element(3,
                                          _vtable_Family@3_0),
    _type_Family_Plain@5_0 = erlang:element(1,
                                            _vtable_Family@3_0),
    _where_Family@6_0 =
        _type_Family_Plain@5_0(_witness_Family@2_0),
    _witness_Show@7_0 = erlang:element(1,
                                       _where_Family@6_0),
    _vtable_Show@8_0 = erlang:element(1, _witness_Show@7_0),
    _fn_Show_show@9_0 = erlang:element(1, _vtable_Show@8_0),
    begin
        _fn_Show_show@9_0(_witness_Show@7_0,
                          _fn_Family_plain@4_0(_witness_Family@2_0,
                                               _f@1_0,
                                               <<"x"/utf8>>))
    end.

show_member(_witnesses_where_show_member@0_0, _f@1_0) ->
    _witness_Family@2_0 = erlang:element(1,
                                         _witnesses_where_show_member@0_0),
    _vtable_Family@3_0 = erlang:element(1,
                                        _witness_Family@2_0),
    _fn_Family_member@4_0 = erlang:element(4,
                                           _vtable_Family@3_0),
    _type_Family_Member@5_0 = erlang:element(2,
                                             _vtable_Family@3_0),
    _witness_Show@6_0 =
        'Elixir.Patina.AssocBound.$impl.Core.Int':'AssocBound::Show::$patina_witness'(),
    _witnesses_where_Family_member@7_0 =
        {_witness_Show@6_0},
    _where_Family@8_0 =
        _type_Family_Member@5_0(_witness_Family@2_0,
                                _witnesses_where_Family_member@7_0),
    _witness_Show@9_0 = erlang:element(1,
                                       _where_Family@8_0),
    _vtable_Show@10_0 = erlang:element(1,
                                       _witness_Show@9_0),
    _fn_Show_show@11_0 = erlang:element(1,
                                        _vtable_Show@10_0),
    begin
        _fn_Show_show@11_0(_witness_Show@9_0,
                           _fn_Family_member@4_0(_witness_Family@2_0,
                                                 _witnesses_where_Family_member@7_0,
                                                 _f@1_0,
                                                 3))
    end.

wrapper_plain() ->
    _witness_Family@0_0 =
        'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::$patina_witness'(),
    _witnesses_where_show_plain@1_0 = {_witness_Family@0_0},
    begin
        show_plain(_witnesses_where_show_plain@1_0,
                   'Elixir.Patina.AssocBound.Wrapper')
    end.

wrapper_member() ->
    _witness_Family@0_0 =
        'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::$patina_witness'(),
    _witnesses_where_show_member@1_0 =
        {_witness_Family@0_0},
    begin
        show_member(_witnesses_where_show_member@1_0,
                    'Elixir.Patina.AssocBound.Wrapper')
    end.
