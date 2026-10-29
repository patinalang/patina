-file("tests/ui/src/generics/assoc_items/the_problem"
      "__01.pt",
      4).

-module('Elixir.Patina.TheProblem01').

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

difference(_witnesses_where_difference@0_0,
           _container@1_0) ->
    _witness_Contains@2_0 = erlang:element(1,
                                           _witnesses_where_difference@0_0),
    _vtable_Contains@3_0 = erlang:element(1,
                                          _witness_Contains@2_0),
    _fn_Contains_last@4_0 = erlang:element(3,
                                           _vtable_Contains@3_0),
    _fn_Contains_first@5_0 = erlang:element(2,
                                            _vtable_Contains@3_0),
    _fn_Contains_last@4_0(_witness_Contains@2_0,
                          _container@1_0)
        -
        _fn_Contains_first@5_0(_witness_Contains@2_0,
                               _container@1_0).

main() ->
    _witness_Contains@0_0 =
        'Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::$patina_witness'(),
    _witnesses_where_difference@1_0 =
        {_witness_Contains@0_0},
    _number_1@2_0 = 3,
    _number_2@3_0 = 10,
    _container@4_0 =
        'Elixir.Patina.TheProblem01.Container':'$patina_constructor__Container'(_number_1@2_0,
                                                                                _number_2@3_0),
    io:put_chars(begin
                     _element@8_0 = begin
                                        _element@5_0 = io_lib:format("~ktp",
                                                                     [_number_1@2_0]),
                                        _element@6_0 = io_lib:format("~ktp",
                                                                     [_number_2@3_0]),
                                        _element@7_0 = io_lib:format("~ktp",
                                                                     ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::contains'(_witness_Contains@0_0,
                                                                                                                                                          _container@4_0,
                                                                                                                                                          _number_1@2_0,
                                                                                                                                                          _number_2@3_0)]),
                                        [<<"Does container contain "/utf8>>,
                                         _element@5_0,
                                         <<" and "/utf8>>,
                                         _element@6_0,
                                         <<": "/utf8>>,
                                         _element@7_0]
                                    end,
                     [_element@8_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@10_0 = begin
                                         _element@9_0 = io_lib:format("~ktp",
                                                                      ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::first'(_witness_Contains@0_0,
                                                                                                                                                        _container@4_0)]),
                                         [<<"First number: "/utf8>>,
                                          _element@9_0]
                                     end,
                     [_element@10_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@12_0 = begin
                                         _element@11_0 = io_lib:format("~ktp",
                                                                       ['Elixir.Patina.TheProblem01.Container':'TheProblem01::Contains<int, int>::last'(_witness_Contains@0_0,
                                                                                                                                                        _container@4_0)]),
                                         [<<"Last number: "/utf8>>,
                                          _element@11_0]
                                     end,
                     [_element@12_0]
                 end),
    io:nl(),
    io:put_chars(begin
                     _element@14_0 = begin
                                         _element@13_0 = io_lib:format("~ktp",
                                                                       [difference(_witnesses_where_difference@1_0,
                                                                                   _container@4_0)]),
                                         [<<"The difference is: "/utf8>>,
                                          _element@13_0]
                                     end,
                     [_element@14_0]
                 end),
    io:nl(),
    {}.
