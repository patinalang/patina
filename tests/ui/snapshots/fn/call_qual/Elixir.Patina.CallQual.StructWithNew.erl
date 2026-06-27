-file("tests/ui/src/fn/call_qual.pt", 1).
-module('Elixir.Patina.CallQual.StructWithNew').
-compile([no_auto_import]).
-export(['__info__'/1,'__struct__'/0,'__struct__'/1,new/1,public_new/1]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    [#{field => value, default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.CallQual.StructWithNew',
      value => nil}.
'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun({Key, Value}, Acc) ->
                                Acc#{Key := Value}
                         end).
new(_var_0) ->
    _ = public_new(_var_0),
    _ = public_new(_var_0),
    'Elixir.Patina.CallQual.StructWithNew':public_new(_var_0).
public_new(_var_0) ->
    priv_new(_var_0).
priv_new(_var_0) ->
    ('__struct__'())#{value := _var_0}.
