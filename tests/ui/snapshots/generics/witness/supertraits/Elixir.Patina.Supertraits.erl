-file("tests/ui/src/generics/witness/supertraits.pt",
      1).

-module('Elixir.Patina.Supertraits').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'supertraits::Greet::calls_supertrait'/2,
         'supertraits::Loud::shout'/2,
         'supertraits::Library::count'/4,
         bob_intro/0,
         robot_intro/0,
         bob_intro_loud/0,
         books_count/1]).

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

'supertraits::Greet::calls_supertrait'(_witness_Greet@0_0,
                                       _self@1_0) ->
    _supertraits_Greet@2_0 = erlang:element(2,
                                            _witness_Greet@0_0),
    _vtable_Greet@3_0 = erlang:element(1,
                                       _witness_Greet@0_0),
    _witness_Named@4_0 = erlang:element(1,
                                        _supertraits_Greet@2_0),
    _vtable_Named@5_0 = erlang:element(1,
                                       _witness_Named@4_0),
    _fn_Named_name@6_0 = erlang:element(1,
                                        _vtable_Named@5_0),
    _fn_Greet_greeting@7_0 = erlang:element(1,
                                            _vtable_Greet@3_0),
    _element@8_0 =
        _fn_Greet_greeting@7_0(_witness_Greet@0_0, _self@1_0),
    _element@9_0 = _fn_Named_name@6_0(_witness_Named@4_0,
                                      _self@1_0),
    {_element@8_0, _element@9_0}.

'supertraits::Loud::shout'(_witness_Loud@0_0,
                           _self@1_0) ->
    _supertraits_Loud@2_0 = erlang:element(2,
                                           _witness_Loud@0_0),
    _witness_Greet@3_0 = erlang:element(1,
                                        _supertraits_Loud@2_0),
    _supertraits_Greet@4_0 = erlang:element(2,
                                            _witness_Greet@3_0),
    _vtable_Greet@5_0 = erlang:element(1,
                                       _witness_Greet@3_0),
    _witness_Named@6_0 = erlang:element(1,
                                        _supertraits_Greet@4_0),
    _vtable_Named@7_0 = erlang:element(1,
                                       _witness_Named@6_0),
    _fn_Named_name@8_0 = erlang:element(1,
                                        _vtable_Named@7_0),
    _fn_Greet_calls_supertrait@9_0 = erlang:element(2,
                                                    _vtable_Greet@5_0),
    _element@10_0 = _fn_Named_name@8_0(_witness_Named@6_0,
                                       _self@1_0),
    _element@11_0 =
        _fn_Greet_calls_supertrait@9_0(_witness_Greet@3_0,
                                       _self@1_0),
    {_element@10_0, _element@11_0}.

via_subtrait_bound(_witnesses_where_via_subtrait_bound@0_0,
                   _x@1_0) ->
    _witness_Greet@2_0 = erlang:element(1,
                                        _witnesses_where_via_subtrait_bound@0_0),
    _supertraits_Greet@3_0 = erlang:element(2,
                                            _witness_Greet@2_0),
    _vtable_Greet@4_0 = erlang:element(1,
                                       _witness_Greet@2_0),
    _witness_Named@5_0 = erlang:element(1,
                                        _supertraits_Greet@3_0),
    _vtable_Named@6_0 = erlang:element(1,
                                       _witness_Named@5_0),
    _fn_Named_name@7_0 = erlang:element(1,
                                        _vtable_Named@6_0),
    _fn_Greet_calls_supertrait@8_0 = erlang:element(2,
                                                    _vtable_Greet@4_0),
    _element@9_0 = _fn_Named_name@7_0(_witness_Named@5_0,
                                      _x@1_0),
    _element@10_0 =
        _fn_Greet_calls_supertrait@8_0(_witness_Greet@2_0,
                                       _x@1_0),
    {_element@9_0, _element@10_0}.

via_indirect_supertrait(_witnesses_where_via_indirect_supertrait@0_0,
                        _x@1_0) ->
    _witness_Loud@2_0 = erlang:element(1,
                                       _witnesses_where_via_indirect_supertrait@0_0),
    _supertraits_Loud@3_0 = erlang:element(2,
                                           _witness_Loud@2_0),
    _vtable_Loud@4_0 = erlang:element(1, _witness_Loud@2_0),
    _witness_Greet@5_0 = erlang:element(1,
                                        _supertraits_Loud@3_0),
    _supertraits_Greet@6_0 = erlang:element(2,
                                            _witness_Greet@5_0),
    _witness_Named@7_0 = erlang:element(1,
                                        _supertraits_Greet@6_0),
    _vtable_Named@8_0 = erlang:element(1,
                                       _witness_Named@7_0),
    _fn_Named_name@9_0 = erlang:element(1,
                                        _vtable_Named@8_0),
    _fn_Loud_shout@10_0 = erlang:element(1,
                                         _vtable_Loud@4_0),
    _element@11_0 = _fn_Named_name@9_0(_witness_Named@7_0,
                                       _x@1_0),
    _element@12_0 = _fn_Loud_shout@10_0(_witness_Loud@2_0,
                                        _x@1_0),
    {_element@11_0, _element@12_0}.

'supertraits::Library::count'(_witness_Library@0_0,
                              _witnesses_where_Library_count@1_0, _self@2_0,
                              _p@3_0) ->
    _supertraits_Library@4_0 = erlang:element(2,
                                              _witness_Library@0_0),
    _witness_Shelf@5_0 = erlang:element(1,
                                        _supertraits_Library@4_0),
    _vtable_Shelf@6_0 = erlang:element(1,
                                       _witness_Shelf@5_0),
    _fn_Shelf_size@7_0 = erlang:element(1,
                                        _vtable_Shelf@6_0),
    _witness_Pred@8_0 = erlang:element(1,
                                       _witnesses_where_Library_count@1_0),
    _vtable_Pred@9_0 = erlang:element(1, _witness_Pred@8_0),
    _fn_Pred_test@10_0 = erlang:element(1,
                                        _vtable_Pred@9_0),
    _element@11_0 = _fn_Shelf_size@7_0(_witness_Shelf@5_0,
                                       _self@2_0),
    _element@12_0 = _fn_Pred_test@10_0(_witness_Pred@8_0,
                                       _p@3_0,
                                       _fn_Shelf_size@7_0(_witness_Shelf@5_0,
                                                          _self@2_0)),
    {_element@11_0, _element@12_0}.

count_in(_witnesses_where_count_in@0_0, _l@1_0) ->
    _witness_Library@2_0 = erlang:element(1,
                                          _witnesses_where_count_in@0_0),
    _vtable_Library@3_0 = erlang:element(1,
                                         _witness_Library@2_0),
    _fn_Library_count@4_0 = erlang:element(1,
                                           _vtable_Library@3_0),
    _witness_Pred@5_0 =
        'Elixir.Patina.Supertraits.IsEven':'supertraits::Pred::$patina_witness'(),
    _witnesses_where_Library_count@6_0 =
        {_witness_Pred@5_0},
    _fn_Library_count@4_0(_witness_Library@2_0,
                          _witnesses_where_Library_count@6_0,
                          _l@1_0,
                          'Elixir.Patina.Supertraits.IsEven').

bob_intro() ->
    _witness_Greet@0_0 =
        'Elixir.Patina.Supertraits.Bob':'supertraits::Greet::$patina_witness'(),
    _witnesses_where_via_subtrait_bound@1_0 =
        {_witness_Greet@0_0},
    via_subtrait_bound(_witnesses_where_via_subtrait_bound@1_0,
                       'Elixir.Patina.Supertraits.Bob').

robot_intro() ->
    _witness_Greet@0_0 =
        'Elixir.Patina.Supertraits.Robot':'supertraits::Greet::$patina_witness'(),
    _witnesses_where_via_subtrait_bound@1_0 =
        {_witness_Greet@0_0},
    via_subtrait_bound(_witnesses_where_via_subtrait_bound@1_0,
                       1).

bob_intro_loud() ->
    _witness_Loud@0_0 =
        'Elixir.Patina.Supertraits.Bob':'supertraits::Loud::$patina_witness'(),
    _witnesses_where_via_indirect_supertrait@1_0 =
        {_witness_Loud@0_0},
    via_indirect_supertrait(_witnesses_where_via_indirect_supertrait@1_0,
                            'Elixir.Patina.Supertraits.Bob').

books_count(_n@0_0) ->
    _witness_Library@1_0 =
        'Elixir.Patina.Supertraits.Books':'supertraits::Library::$patina_witness'(),
    _witnesses_where_count_in@2_0 = {_witness_Library@1_0},
    count_in(_witnesses_where_count_in@2_0, _n@0_0).
