-file("tests/ui/src/generics/witness/bound_generic.pt",
      3).

-module('Elixir.Patina.BoundGeneric.Holder').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BoundGeneric::Tr::method'/2,
         'BoundGeneric::Tr::$patina_witness'/1]).

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

'BoundGeneric::Tr::method'(_witness_self@0_0 = {_,
                                                {},
                                                {_witness_where_clause@1_0}},
                           _self@2_0) ->
    begin
        io:put_chars([[<<"Tr::method("/utf8>>,
                       io_lib:format("~ktp", [_self@2_0]),
                       <<")"/utf8>>]]),
        io:nl(),
        {}
    end,
    (erlang:element(1,
                    erlang:element(1,
                                   _witness_where_clause@1_0)))(_witness_where_clause@1_0,
                                                                _self@2_0).

'BoundGeneric::Tr::$patina_witness'(_witness_bounds@1_0 =
                                        {_witness_bound@0_0}) ->
    {{fun 'Elixir.Patina.BoundGeneric.Holder':'BoundGeneric::Tr::method'/2},
     {},
     _witness_bounds@1_0}.
