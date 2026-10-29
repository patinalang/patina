-file("tests/ui/src/primitives/ranges.pt", 1).

-module('Elixir.Patina.Ranges').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         half_open/2,
         closed/2,
         half_open_bounds/0,
         closed_bounds/0,
         char_range/0,
         destructure/1]).

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

half_open(_start@0_0, _end@1_0) ->
    'Elixir.Patina.Core.Range':'$patina_constructor__Range'(_start@0_0,
                                                            _end@1_0).

closed(_start@0_0, _end@1_0) ->
    'Elixir.Patina.Core.RangeInclusive':new(_start@0_0,
                                            _end@1_0).

half_open_bounds() ->
    _range@0_0 =
        'Elixir.Patina.Core.Range':'$patina_constructor__Range'(1,
                                                                101),
    _element@1_0 =
        _range@0_0#'Elixir.Patina.Core.Range':'Range'.start,
    _element@2_0 =
        _range@0_0#'Elixir.Patina.Core.Range':'Range'.'end',
    {_element@1_0, _element@2_0}.

closed_bounds() ->
    _range@0_0 = 'Elixir.Patina.Core.RangeInclusive':new(1,
                                                         100),
    _element@1_0 =
        _range@0_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.start,
    _element@2_0 =
        _range@0_0#'Elixir.Patina.Core.RangeInclusive':'RangeInclusive'.'end',
    {_element@1_0, _element@2_0}.

char_range() ->
    'Elixir.Patina.Core.RangeInclusive':new($a, $z).

destructure(_range@0_0) ->
    #'Elixir.Patina.Core.Range':'Range'{start = _start@1_0,
                                        'end' = _end@2_0} =
        _range@0_0,
    _end@2_0 - _start@1_0.
