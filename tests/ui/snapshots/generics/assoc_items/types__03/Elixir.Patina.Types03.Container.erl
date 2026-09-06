-file("tests/ui/src/generics/assoc_items/types__03.pt",
      4).

-module('Elixir.Patina.Types03.Container').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         'Types03::Contains::contains'/4,
         'Types03::Contains::first'/2,
         'Types03::Contains::last'/2]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => '0', default => nil, required => false},
     #{field => '1', default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Types03.Container',
      '0' => nil, '1' => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

'Types03::Contains::contains'(_Self@0_0, _self@1_0,
                              _number_1@2_0, _number_2@3_0) ->
    erlang:map_get('0', _self@1_0) =:= _number_1@2_0 andalso
        erlang:map_get('1', _self@1_0) =:= _number_2@3_0.

'Types03::Contains::first'(_Self@0_0, _self@1_0) ->
    erlang:map_get('0', _self@1_0).

'Types03::Contains::last'(_Self@0_0, _self@1_0) ->
    erlang:map_get('1', _self@1_0).
