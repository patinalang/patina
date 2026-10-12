-file("tests/ui/src/generics/witness/bound_generic.pt",
      5).

-module('Elixir.Patina.BoundGeneric.Number').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BoundGeneric::Bound::b'/2,
         'BoundGeneric::Bound::$patina_witness'/0]).

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

'BoundGeneric::Bound::b'(_witness_self@0_0 = {_,
                                              {},
                                              {}},
                         _self@1_0) ->
    begin
        io:put_chars([[<<"Bound::b("/utf8>>,
                       io_lib:format("~ktp", [_self@1_0]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    _self@1_0.

'BoundGeneric::Bound::$patina_witness'() ->
    {{fun 'Elixir.Patina.BoundGeneric.Number':'BoundGeneric::Bound::b'/2},
     {},
     {}}.
