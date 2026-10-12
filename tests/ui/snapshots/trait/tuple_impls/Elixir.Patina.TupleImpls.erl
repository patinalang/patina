-file("tests/ui/src/trait/tuple_impls.pt", 1).

-module('Elixir.Patina.TupleImpls').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         show_unit/0,
         show_pair/2,
         show_generic_all/2]).

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

show_generic(_witness_bounds@1_0 = {_witness_bound@0_0},
             _t@2_0) ->
    (erlang:element(1,
                    erlang:element(1,
                                   _witness_bound@0_0)))(_witness_bound@0_0,
                                                         _t@2_0).

show_unit() ->
    'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<()>'('Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<()>'(),
                                                                             {}).

show_pair(_n@0_0, _s@1_0) ->
    'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::show<(_, _)>'('Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<(_, _)>'({'Elixir.Patina.TupleImpls.$impl.Core.Int':'TupleImpls::Show::$patina_witness'(),
                                                                                                                                                                          'Elixir.Patina.TupleImpls.$impl.Core.Str':'TupleImpls::Show::$patina_witness'()}),
                                                                                 {_n@0_0,
                                                                                  _s@1_0}).

show_generic_all(_n@0_0, _s@1_0) ->
    <<<<(show_generic({'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<(_,)>'({'Elixir.Patina.TupleImpls.$impl.Core.Int':'TupleImpls::Show::$patina_witness'()})},
                      {_n@0_0}))/binary,
        (show_generic({'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<(_, _)>'({'Elixir.Patina.TupleImpls.$impl.Core.Int':'TupleImpls::Show::$patina_witness'(),
                                                                                                                'Elixir.Patina.TupleImpls.$impl.Core.Str':'TupleImpls::Show::$patina_witness'()})},
                      {_n@0_0, _s@1_0}))/binary>>/binary,
      (show_generic({'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<(_, _, _)>'({'Elixir.Patina.TupleImpls.$impl.Core.Int':'TupleImpls::Show::$patina_witness'(),
                                                                                                                 'Elixir.Patina.TupleImpls.$impl.Core.Str':'TupleImpls::Show::$patina_witness'(),
                                                                                                                 'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<(_, _)>'({'Elixir.Patina.TupleImpls.$impl.Core.Int':'TupleImpls::Show::$patina_witness'(),
                                                                                                                                                                                                          'Elixir.Patina.TupleImpls.$impl.Core.Tuple':'TupleImpls::Show::$patina_witness<()>'()})})},
                    {_n@0_0, _s@1_0, {_n@0_0, {}}}))/binary>>.
