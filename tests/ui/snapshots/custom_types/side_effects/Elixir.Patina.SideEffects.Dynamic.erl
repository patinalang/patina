-file("tests/ui/src/custom_types/side_effects.pt", 105).

-module('Elixir.Patina.SideEffects.Dynamic').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export([fields/1]).

fields(_value@0_0) ->
    _value@1_0 = 'Elixir.Patina.SideEffects':effect(field,
                                                    _value@0_0),
    case _value@1_0 of
        _value@2_0
            when (erlang:is_tuple(_value@2_0) andalso
                      erlang:tuple_size(_value@2_0) =:= 2)
                     andalso erlang:element(1, _value@2_0) =:= dynamic ->
            {dynamic, _value@2_0};
        _value@2_0 when erlang:is_binary(_value@2_0) ->
            {dynamic, _value@2_0};
        _value@2_0 -> _value@2_0
    end.
