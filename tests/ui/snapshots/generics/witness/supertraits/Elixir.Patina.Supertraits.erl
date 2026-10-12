-file("tests/ui/src/generics/witness/supertraits.pt",
      1).

-module('Elixir.Patina.Supertraits').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Supertraits::Greet::calls_supertrait'/2,
         'Supertraits::Loud::shout'/2,
         'Supertraits::Library::count'/4,
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

'Supertraits::Greet::calls_supertrait'(_witness@0_0,
                                       _self@1_0) ->
    {(erlang:element(1,
                     erlang:element(1, _witness@0_0)))(_witness@0_0,
                                                       _self@1_0),
     begin
         _witness@2_0 = erlang:element(1,
                                       erlang:element(2, _witness@0_0)),
         (erlang:element(1,
                         erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                           _self@1_0)
     end}.

'Supertraits::Loud::shout'(_witness@0_0, _self@1_0) ->
    {begin
         _witness@2_0 = erlang:element(1,
                                       erlang:element(2,
                                                      erlang:element(1,
                                                                     erlang:element(2,
                                                                                    _witness@0_0)))),
         (erlang:element(1,
                         erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                           _self@1_0)
     end,
     begin
         _witness@3_0 = erlang:element(1,
                                       erlang:element(2, _witness@0_0)),
         (erlang:element(2,
                         erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                           _self@1_0)
     end}.

via_subtrait_bound(_witness@0_0, _x@1_0) ->
    {begin
         _witness@2_0 = erlang:element(1,
                                       erlang:element(2,
                                                      erlang:element(1,
                                                                     _witness@0_0))),
         (erlang:element(1,
                         erlang:element(1, _witness@2_0)))(_witness@2_0, _x@1_0)
     end,
     begin
         _witness@3_0 = erlang:element(1, _witness@0_0),
         (erlang:element(2,
                         erlang:element(1, _witness@3_0)))(_witness@3_0, _x@1_0)
     end}.

via_indirect_supertrait(_witness@0_0, _x@1_0) ->
    {begin
         _witness@2_0 = erlang:element(1,
                                       erlang:element(2,
                                                      erlang:element(1,
                                                                     erlang:element(2,
                                                                                    erlang:element(1,
                                                                                                   _witness@0_0))))),
         (erlang:element(1,
                         erlang:element(1, _witness@2_0)))(_witness@2_0, _x@1_0)
     end,
     begin
         _witness@3_0 = erlang:element(1, _witness@0_0),
         (erlang:element(1,
                         erlang:element(1, _witness@3_0)))(_witness@3_0, _x@1_0)
     end}.

'Supertraits::Library::count'(_witness@0_0,
                              _witness@1_0, _self@2_0, _p@3_0) ->
    {begin
         _witness@4_0 = erlang:element(1,
                                       erlang:element(2, _witness@0_0)),
         (erlang:element(1,
                         erlang:element(1, _witness@4_0)))(_witness@4_0,
                                                           _self@2_0)
     end,
     begin
         _witness@6_0 = erlang:element(1, _witness@1_0),
         (erlang:element(1,
                         erlang:element(1, _witness@6_0)))(_witness@6_0,
                                                           _p@3_0,
                                                           begin
                                                               _witness@5_0 =
                                                                   erlang:element(1,
                                                                                  erlang:element(2,
                                                                                                 _witness@0_0)),
                                                               (erlang:element(1,
                                                                               erlang:element(1,
                                                                                              _witness@5_0)))(_witness@5_0,
                                                                                                              _self@2_0)
                                                           end)
     end}.

count_in(_witness@0_0, _l@1_0) ->
    begin
        _witness@2_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                          {'Elixir.Patina.Supertraits.IsEven':'Supertraits::Pred::$patina_witness'()},
                                                          _l@1_0,
                                                          'Elixir.Patina.Supertraits.IsEven')
    end.

bob_intro() ->
    via_subtrait_bound({'Elixir.Patina.Supertraits.Bob':'Supertraits::Greet::$patina_witness'()},
                       'Elixir.Patina.Supertraits.Bob').

robot_intro() ->
    via_subtrait_bound({'Elixir.Patina.Supertraits.Robot':'Supertraits::Greet::$patina_witness'()},
                       1).

bob_intro_loud() ->
    via_indirect_supertrait({'Elixir.Patina.Supertraits.Bob':'Supertraits::Loud::$patina_witness'()},
                            'Elixir.Patina.Supertraits.Bob').

books_count(_n@0_0) ->
    count_in({'Elixir.Patina.Supertraits.Books':'Supertraits::Library::$patina_witness'()},
             _n@0_0).
