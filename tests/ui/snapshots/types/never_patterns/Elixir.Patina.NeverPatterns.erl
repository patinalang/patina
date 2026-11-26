-file("tests/ui/src/types/never_patterns.pt", 3).

-module('Elixir.Patina.NeverPatterns').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         option_none_first/0,
         option_some_first/0,
         or_pattern_b/1,
         or_pattern_c/0,
         inlined_none/0,
         generic_some/1,
         construct_some/0,
         no_possible_arms/1]).

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

option_none_first() ->
    _o@0_0 = nil,
    case _o@0_0 of nil -> 1 end.

option_some_first() ->
    _o@0_0 = nil,
    case _o@0_0 of nil -> 1 end.

e_value(_e@0_0) ->
    case [] of
        {'$patina_label_0', _n@3_0} -> _n@3_0;
        _ ->
            case _e@0_0 of
                _value@2_0 = _n@1_0
                    when erlang:is_integer(_value@2_0) ->
                    '$patina_marker':jump('$patina_label_0', _n@1_0),
                    [];
                'Elixir.Patina.NeverPatterns.E.C' -> 0
            end
    end.

or_pattern_b(_n@0_0) -> e_value(_n@0_0).

or_pattern_c() ->
    e_value('Elixir.Patina.NeverPatterns.E.C').

is_some(_o@0_0) ->
    case _o@0_0 of
        {some, _} -> true;
        _value@1_0 = _
            when not (_value@1_0 =:= nil),
                 not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= some) ->
            true;
        nil -> false
    end.

inlined_none() ->
    _o@0_0 = nil,
    _o@1_0 = _o@0_0,
    case _o@1_0 of nil -> false end.

generic_some(_n@0_0) ->
    _o@1_0 = _n@0_0,
    case _o@1_0 of
        _value@2_0 = _ when erlang:is_integer(_value@2_0) ->
            true;
        nil -> false
    end.

construct_some() ->
    _0@0_0 =
        erlang:error('Elixir.Patina.Core.Panic':new(<<"boom"/utf8>>),
                     none,
                     [{error_info,
                       #{module => 'Elixir.Patina.Core.Panic'}}]),
    _0@0_0.

no_possible_arms(_r@0_0) -> case _r@0_0 of  end.
