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
    _witness_Animal@1_0 =
        'Trait01::Animal::$patina_witness'(),
    case is_naked(_self@0_0) of
        true ->
            io:put_chars(begin
                             _element@3_0 = begin
                                                _element@2_0 =
                                                    'Trait01::Animal::name'(_witness_Animal@1_0,
                                                                            _self@0_0),
                                                [_element@2_0,
                                                 <<" is already naked..."/utf8>>]
                                            end,
                             [_element@3_0]
                         end),
            io:nl(),
            _self@0_0;
        _ ->
            io:put_chars(begin
                             _element@5_0 = begin
                                                _element@4_0 =
                                                    _self@0_0#'Sheep'.name,
                                                [_element@4_0,
                                                 <<" gets a haircut!"/utf8>>]
                                            end,
                             [_element@5_0]
                         end),
            io:nl(),
            _self@0_0#'Sheep'{naked = true}
    end.

'Trait01::Animal::new'(_witness_Animal@0_0,
                       _name@1_0) ->
    #'Sheep'{naked = false, name = _name@1_0}.

'Trait01::Animal::name'(_witness_Animal@0_0,
                        _self@1_0) ->
    _self@1_0#'Sheep'.name.

'Trait01::Animal::noise'(_witness_Animal@0_0,
                         _self@1_0) ->
    case is_naked(_self@1_0) of
        true -> <<"baaaaah?"/utf8>>;
        _ -> <<"baaaaah!"/utf8>>
    end.

'Trait01::Animal::talk'(_witness_Animal@0_0,
                        _self@1_0) ->
    io:put_chars(begin
                     _element@4_0 = begin
                                        _element@2_0 = _self@1_0#'Sheep'.name,
                                        _element@3_0 =
                                            'Trait01::Animal::noise'(_witness_Animal@0_0,
                                                                     _self@1_0),
                                        [_element@2_0,
                                         <<" pauses briefly... "/utf8>>,
                                         _element@3_0]
                                    end,
                     [_element@4_0]
                 end),
    io:nl(),
    {}.

'Trait01::Animal::$patina_witness'() ->
    {{fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::new'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::name'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::noise'/2,
      fun 'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::talk'/2},
     {},
     {}}.
