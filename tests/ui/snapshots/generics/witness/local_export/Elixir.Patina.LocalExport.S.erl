-file("tests/ui/src/generics/witness/local_export.pt",
      3).

-module('Elixir.Patina.LocalExport.S').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'local_export::Priv::p'/2,
         'local_export::Priv::$patina_witness'/0,
         q/1]).

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

'local_export::Priv::p'(_witness_Priv@0_0, _self@1_0) ->
    io:put_chars(begin
                     _element@3_0 = begin
                                        _element@2_0 = io_lib:format("~ktp",
                                                                     [_self@1_0]),
                                        [<<"Priv::p("/utf8>>,
                                         _element@2_0,
                                         <<")"/utf8>>]
                                    end,
                     [_element@3_0]
                 end),
    io:nl(),
    {}.

'local_export::Priv::$patina_witness'() ->
    {{fun 'Elixir.Patina.LocalExport.S':'local_export::Priv::p'/2},
     {},
     {}}.

q(_self@0_0) ->
    _witness_Priv@1_0 =
        'local_export::Priv::$patina_witness'(),
    'local_export::Priv::p'(_witness_Priv@1_0, _self@0_0).
