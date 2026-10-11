-file("tests/ui/src/generics/assoc_items/types__03.pt",
      4).

-module('Elixir.Patina.Types03').

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

difference(_witness@1_0 = {_witness@0_0},
           _container@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Types03::Contains::last'(_witness@0_0,
                                                             _container@2_0)
        -
        (erlang:element(2,
                        _witness@0_0)):'Types03::Contains::first'(_witness@0_0,
                                                                  _container@2_0).

main() ->
    _number_1@0_0 = 3,
    _number_2@1_0 = 10,
    _container@2_0 =
        'Elixir.Patina.Types03.Container':'$patina_constructor__Container'(_number_1@0_0,
                                                                           _number_2@1_0),
    begin
        io:put_chars([[<<"Does container contain "/utf8>>,
                       io_lib:format("~ktp", [_number_1@0_0]),
                       <<" and "/utf8>>,
                       io_lib:format("~ktp", [_number_2@1_0]),
                       <<": "/utf8>>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::contains'({'impl Types03::Contains for Types03::Container',
                                                                                                       'Elixir.Patina.Types03.Container'},
                                                                                                      _container@2_0,
                                                                                                      _number_1@0_0,
                                                                                                      _number_2@1_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"First number: "/utf8>>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::first'({'impl Types03::Contains for Types03::Container',
                                                                                                    'Elixir.Patina.Types03.Container'},
                                                                                                   _container@2_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"Last number: "/utf8>>,
                       io_lib:format("~ktp",
                                     ['Elixir.Patina.Types03.Container':'Types03::Contains::last'({'impl Types03::Contains for Types03::Container',
                                                                                                   'Elixir.Patina.Types03.Container'},
                                                                                                  _container@2_0)])]]),
        io:nl(),
        {}
    end,
    begin
        io:put_chars([[<<"The difference is: "/utf8>>,
                       io_lib:format("~ktp",
                                     [difference({{'impl Types03::Contains for Types03::Container',
                                                   'Elixir.Patina.Types03.Container'}},
                                                 _container@2_0)])]]),
        io:nl(),
        {}
    end.
