-file("tests/ui/src/generics/new_types__01.pt", 4).

-module('Elixir.Patina.NewTypes01.Years').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         to_days/1]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => '0', default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.NewTypes01.Years',
      '0' => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

to_days(_self@0_0) ->
    ('Elixir.Patina.NewTypes01.Days':'__struct__'())#{'0' :=
                                                          erlang:map_get('0',
                                                                         _self@0_0)
                                                              * 365}.
