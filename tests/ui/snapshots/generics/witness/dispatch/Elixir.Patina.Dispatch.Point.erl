-file("tests/ui/src/generics/witness/dispatch.pt", 1).

-module('Elixir.Patina.Dispatch.Point').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['Point']).

-export(['__info__'/1,
         '$patina_constructor__Point'/2,
         'Dispatch::Area::area'/2,
         'Dispatch::Area::$patina_witness'/0]).

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

-record #'Point'{x, y}.

'$patina_constructor__Point'(_x@0_0, _y@1_0) ->
    #'Point'{x = _x@0_0, y = _y@1_0}.

'Dispatch::Area::area'(_witness_Area@0_0, _self@1_0) ->
    begin
        io:put_chars(begin
                         _element@5_0 = begin
                                            _element@2_0 =
                                                <<"Area::area("/utf8>>,
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
    end,
    _self@1_0#'Point'.x * _self@1_0#'Point'.y.

'Dispatch::Area::$patina_witness'() ->
    {{fun 'Elixir.Patina.Dispatch.Point':'Dispatch::Area::area'/2},
     {},
     {}}.
