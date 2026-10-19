-file("tests/ui/src/generics/witness/assoc_bound.pt",
      50).

-module('Elixir.Patina.AssocBound.Wrapper').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'AssocBound::Family::$patina_type::Plain'/1,
         'AssocBound::Family::$patina_type::Member'/2,
         'AssocBound::Family::plain'/3,
         'AssocBound::Family::member'/4,
         'AssocBound::Family::$patina_witness'/0]).

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

'AssocBound::Family::$patina_type::Plain'(_witness_Family@0_0) ->
    _witness_Show@1_0 =
        {{fun 'Elixir.Patina.AssocBound.$impl.Core.Int':'AssocBound::Show::show'/2},
         {},
         {}},
    {_witness_Show@1_0}.

'AssocBound::Family::$patina_type::Member'(_witness_Family@0_0,
                                           _witnesses_where_Family_Member@1_0) ->
    _witness_Show@2_0 = erlang:element(1,
                                       _witnesses_where_Family_Member@1_0),
    _witness_Show@3_0 =
        {{fun 'Elixir.Patina.AssocBound.Pair':'AssocBound::Show::show'/2},
         {},
         {_witness_Show@2_0}},
    {_witness_Show@3_0}.

'AssocBound::Family::plain'(_witness_Family@0_0,
                            _self@1_0, __t@2_0) ->
    7.

'AssocBound::Family::member'(_witness_Family@0_0,
                             _witnesses_where_Family_member@1_0, _self@2_0,
                             _t@3_0) ->
    _t@3_0.

'AssocBound::Family::$patina_witness'() ->
    {{fun 'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::$patina_type::Plain'/1,
      fun 'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::$patina_type::Member'/2,
      fun 'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::plain'/3,
      fun 'Elixir.Patina.AssocBound.Wrapper':'AssocBound::Family::member'/4},
     {},
     {}}.
