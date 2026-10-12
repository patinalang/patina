-file("tests/ui/src/trait__01.pt", 4).

-module('Elixir.Patina.Trait01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Trait01::Animal::talk'/2,
         main/0]).

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

'Trait01::Animal::talk'(_witness@0_0, _self@1_0) ->
    begin
        io:put_chars(begin
                         _element@5_0 = begin
                                            _element@2_0 = (erlang:element(2,
                                                                           erlang:element(1,
                                                                                          _witness@0_0)))(_witness@0_0,
                                                                                                          _self@1_0),
                                            _element@3_0 = <<" says "/utf8>>,
                                            _element@4_0 = (erlang:element(3,
                                                                           erlang:element(1,
                                                                                          _witness@0_0)))(_witness@0_0,
                                                                                                          _self@1_0),
                                            [_element@2_0,
                                             _element@3_0,
                                             _element@4_0]
                                        end,
                         [_element@5_0]
                     end),
        io:nl(),
        {}
    end.

main() ->
    _dolly@0_0 =
        'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::new'('Elixir.Patina.Trait01.Sheep':'Trait01::Animal::$patina_witness'(),
                                                             <<"Dolly"/utf8>>),
    'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::talk'('Elixir.Patina.Trait01.Sheep':'Trait01::Animal::$patina_witness'(),
                                                          _dolly@0_0),
    'Elixir.Patina.Trait01.Sheep':shear(_dolly@0_0),
    'Elixir.Patina.Trait01.Sheep':'Trait01::Animal::talk'('Elixir.Patina.Trait01.Sheep':'Trait01::Animal::$patina_witness'(),
                                                          _dolly@0_0).
