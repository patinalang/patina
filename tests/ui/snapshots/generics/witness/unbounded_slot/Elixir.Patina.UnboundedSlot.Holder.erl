-file("tests/ui/src/generics/witness/unbounded_slot.pt",
      3).

-module('Elixir.Patina.UnboundedSlot.Holder').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         '__struct__'/0,
         '__struct__'/1,
         'UnboundedSlot::Tr::method'/2]).

'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1}, {module_info, 0}, {module_info, 1}]);
'__info__'(macros) -> [];
'__info__'(struct) ->
    [#{field => value, default => nil, required => false}];
'__info__'(exports_md5) ->
    erlang:md5(erlang:term_to_binary({lists:sort(module_info(exports)),
                                      [],
                                      '__info__'(struct)},
                                     [deterministic]));
'__info__'(deprecated) -> [];
'__info__'(Key) -> module_info(Key).

'__struct__'() ->
    #{'__struct__' => 'Elixir.Patina.UnboundedSlot.Holder',
      value => nil}.

'__struct__'(Fields) ->
    'Elixir.Enum':reduce(Fields,
                         '__struct__'(),
                         fun ({Key, Value}, Acc) -> Acc#{Key := Value} end).

'UnboundedSlot::Tr::method'(_Self@0_0, _self@1_0) ->
    begin
        io:put_chars([[<<"Tr::method(">>,
                       io_lib:format("~ktp", [_self@1_0]),
                       <<")">>]]),
        io:nl(),
        {}
    end.
