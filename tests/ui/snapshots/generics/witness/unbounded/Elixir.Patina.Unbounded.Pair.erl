-file("tests/ui/src/generics/witness/unbounded.pt", 3).

-module('Elixir.Patina.Unbounded.Pair').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         len/1]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => a, default => nil, required => false},
     #{field => b, default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.Unbounded.Pair',
      a => nil, b => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

len(_self@0_0) ->
    begin
        io:put_chars([[<<"Pair::len(">>,
                       io_lib:format("~ktp", [_self@0_0]),
                       <<")">>]]),
        io:nl()
    end,
    2.
