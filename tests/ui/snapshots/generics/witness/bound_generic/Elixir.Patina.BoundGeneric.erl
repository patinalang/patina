-file("tests/ui/src/generics/witness/bound_generic.pt",
      1).

-module('Elixir.Patina.BoundGeneric').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

call(_witness@0_0, _h@1_0) ->
    'Elixir.Patina.BoundGeneric.Holder':'BoundGeneric::Tr::method'('Elixir.Patina.BoundGeneric.Holder':'BoundGeneric::Tr::$patina_witness'({erlang:element(1,
                                                                                                                                                           _witness@0_0)}),
                                                                   _h@1_0).

main() ->
    _h@1_0 = begin _value@0_0 = 5, _value@0_0 end,
    begin
        io:put_chars(begin
                         _element@3_0 = begin
                                            _element@2_0 = io_lib:format("~ktp",
                                                                         [call({'Elixir.Patina.BoundGeneric.Number':'BoundGeneric::Bound::$patina_witness'()},
                                                                               _h@1_0)]),
                                            [_element@2_0]
                                        end,
                         [_element@3_0]
                     end),
        io:nl(),
        {}
    end.
