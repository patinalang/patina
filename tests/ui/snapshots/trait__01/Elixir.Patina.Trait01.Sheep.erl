-file("tests/ui/src/trait__01.pt", 4).

-module('Elixir.Patina.Trait01.Sheep').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Sheep']).

-export(['__info__'/1,
         '$patina_constructor__Sheep'/2,
         shear/1,
         'Trait01::Animal::new'/2,
         'Trait01::Animal::name'/2,
         'Trait01::Animal::noise'/2,
         'Trait01::Animal::talk'/2,
         'Trait01::Animal::$patina_witness'/0]).

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

-record #'Sheep'{naked, name}.

'$patina_constructor__Sheep'(_naked@0_0, _name@1_0) ->
    #'Sheep'{naked = _naked@0_0, name = _name@1_0}.

is_naked(_self@0_0) -> _self@0_0#'Sheep'.naked.

shear(_self@0_0) ->
    case is_naked(_self@0_0) of
        true ->
            begin
                io:put_chars([['Trait01::Animal::name'('Elixir.Patina.Trait01.Sheep':'Trait01::Animal::$patina_witness'(),
                                                       _self@0_0),
                               <<" is already naked..."/utf8>>]]),
                io:nl(),
                {}
            end,
            _self@0_0;
        false ->
            begin
                io:put_chars([[_self@0_0#'Sheep'.name,
                               <<" gets a haircut!"/utf8>>]]),
                io:nl(),
                {}
            end,
            _self@0_0#'Sheep'{naked = true}
    end.

'Trait01::Animal::new'(_witness_self@0_0 = {_, {}, {}},
                       _name@1_0) ->
    #'Sheep'{naked = false, name = _name@1_0}.

'Trait01::Animal::name'(_witness_self@0_0 = {_, {}, {}},
                        _self@1_0) ->
    _self@1_0#'Sheep'.name.

'Trait01::Animal::noise'(_witness_self@0_0 = {_,
                                              {},
                                              {}},
                         _self@1_0) ->
    case is_naked(_self@1_0) of
        true -> <<"baaaaah?"/utf8>>;
        false -> <<"baaaaah!"/utf8>>
    end.

'Trait01::Animal::talk'(_witness_self@0_0 = {_, {}, {}},
                        _self@1_0) ->
    begin
        io:put_chars([[_self@1_0#'Sheep'.name,
                       <<" pauses briefly... "/utf8>>,
                       'Trait01::Animal::noise'(_witness_self@0_0,
                                                _self@1_0)]]),
        io:nl(),
        {}
    end.

'Trait01::Animal::$patina_witness'() ->
    {{fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::new'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::name'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::noise'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::talk'/2},
     {},
     {}}.
