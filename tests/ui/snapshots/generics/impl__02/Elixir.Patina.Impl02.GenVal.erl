-file("tests/ui/src/generics/impl__02.pt", 8).

-module('Elixir.Patina.Impl02.GenVal').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         value/1]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => gen_val, default => nil,
       required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Impl02.GenVal',
      gen_val => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

value(_self@0_0) -> erlang:map_get(gen_val, _self@0_0).
