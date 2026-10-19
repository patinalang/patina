-file("tests/ui/src/generics/witness/forward.pt", 5).

-module('Elixir.Patina.Forward.S').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Forward::Tr::method'/2,
         'Forward::Tr::$patina_witness'/0]).

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

'Forward::Tr::method'(_witness_Tr@0_0, _self@1_0) ->
    begin
        io:put_chars(begin
                         _element@5_0 = begin
                                            _element@2_0 =
                                                <<"Tr::method("/utf8>>,
                                            _element@3_0 = io_lib:format("~ktp",
                                                                         [_self@1_0]),
                                            _element@4_0 = <<")"/utf8>>,
                                            [_element@2_0,
                                             _element@3_0,
                                             _element@4_0]
                                        end,
                         [_element@5_0]
                     end),
        io:nl(),
        {}
    end.

'Forward::Tr::$patina_witness'() ->
    {{fun 'Elixir.Patina.Forward.S':'Forward::Tr::method'/2},
     {},
     {}}.
