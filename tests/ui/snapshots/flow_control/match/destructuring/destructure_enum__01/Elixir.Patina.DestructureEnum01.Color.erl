-file("tests/ui/src/flow_control/match/destructuring"
      "/destructure_enum__01.pt",
      7).

-module('Elixir.Patina.DestructureEnum01.Color').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record(['RGB', 'HSV', 'HSL', 'CMY', 'CMYK']).

-export(['$patina_constructor__RGB'/3,
         '$patina_constructor__HSV'/3,
         '$patina_constructor__HSL'/3,
         '$patina_constructor__CMY'/3,
         '$patina_constructor__CMYK'/4]).

-record #'RGB'{'0', '1', '2'}.

'$patina_constructor__RGB'(_0@0_0, _1@1_0, _2@2_0) ->
    #'RGB'{'0' = _0@0_0, '1' = _1@1_0, '2' = _2@2_0}.

-record #'HSV'{'0', '1', '2'}.

'$patina_constructor__HSV'(_0@0_0, _1@1_0, _2@2_0) ->
    #'HSV'{'0' = _0@0_0, '1' = _1@1_0, '2' = _2@2_0}.

-record #'HSL'{'0', '1', '2'}.

'$patina_constructor__HSL'(_0@0_0, _1@1_0, _2@2_0) ->
    #'HSL'{'0' = _0@0_0, '1' = _1@1_0, '2' = _2@2_0}.

-record #'CMY'{'0', '1', '2'}.

'$patina_constructor__CMY'(_0@0_0, _1@1_0, _2@2_0) ->
    #'CMY'{'0' = _0@0_0, '1' = _1@1_0, '2' = _2@2_0}.

-record #'CMYK'{'0', '1', '2', '3'}.

'$patina_constructor__CMYK'(_0@0_0, _1@1_0, _2@2_0,
                            _3@3_0) ->
    #'CMYK'{'0' = _0@0_0, '1' = _1@1_0, '2' = _2@2_0,
            '3' = _3@3_0}.
