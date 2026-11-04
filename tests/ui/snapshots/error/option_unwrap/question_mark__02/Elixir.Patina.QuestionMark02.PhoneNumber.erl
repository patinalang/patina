-file("tests/ui/src/error/option_unwrap/question_mar"
      "k__02.pt",
      13).

-module('Elixir.Patina.QuestionMark02.PhoneNumber').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['PhoneNumber']).

-export(['__info__'/1,
         '$patina_constructor__PhoneNumber'/2]).

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

-record #'PhoneNumber'{area_code, number}.

'$patina_constructor__PhoneNumber'(_area_code@0_0,
                                   _number@1_0) ->
    #'PhoneNumber'{area_code = _area_code@0_0,
                   number = _number@1_0}.
