-file("tests/ui/src/trait/disambiguating__01.pt", 15).

-module('Elixir.Patina.Disambiguating01.Form').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Form']).

-export(['__info__'/1,
         '$patina_constructor__Form'/2,
         'Disambiguating01::UsernameWidget::get'/2,
         'Disambiguating01::UsernameWidget::$patina_witness'/0,
         'Disambiguating01::AgeWidget::get'/2,
         'Disambiguating01::AgeWidget::$patina_witness'/0]).

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

-record #'Form'{username, age}.

'$patina_constructor__Form'(_username@0_0, _age@1_0) ->
    #'Form'{username = _username@0_0, age = _age@1_0}.

'Disambiguating01::UsernameWidget::get'(_witness_UsernameWidget@0_0,
                                        _self@1_0) ->
    _self@1_0#'Form'.username.

'Disambiguating01::UsernameWidget::$patina_witness'() ->
    {{fun 'Elixir.Patina.Disambiguating01.Form':'Disambiguating01::UsernameWidget::get'/2},
     {},
     {}}.

'Disambiguating01::AgeWidget::get'(_witness_AgeWidget@0_0,
                                   _self@1_0) ->
    _self@1_0#'Form'.age.

'Disambiguating01::AgeWidget::$patina_witness'() ->
    {{fun 'Elixir.Patina.Disambiguating01.Form':'Disambiguating01::AgeWidget::get'/2},
     {},
     {}}.
