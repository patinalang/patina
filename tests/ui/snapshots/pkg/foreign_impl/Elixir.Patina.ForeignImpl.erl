-file("tests/ui/src/pkg/foreign_impl.pt", 2).

-module('Elixir.Patina.ForeignImpl').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         label_size/0,
         local_to_celsius/1,
         fahrenheit_to_celsius/1,
         kelvin_to_celsius/1,
         rankine_to_celsius/1,
         label_shout/0,
         label_whisper/0,
         label_show/0,
         blanket_on_foreign_type/1,
         blanket_on_local_type/0,
         blanket_generic_both/1]).

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

size_of(_witnesses_where_size_of@0_0, _t@1_0) ->
    _witness_Size@2_0 = erlang:element(1,
                                       _witnesses_where_size_of@0_0),
    _vtable_Size@3_0 = erlang:element(1, _witness_Size@2_0),
    _fn_Size_size@4_0 = erlang:element(1, _vtable_Size@3_0),
    _fn_Size_size@4_0(_witness_Size@2_0, _t@1_0).

label_size() ->
    _witness_Size@0_0 =
        'Elixir.Patina.ForeignImpl.$impl.Traits.Label':'foreign_impl::Size::$patina_witness'(),
    _witnesses_where_size_of@1_0 = {_witness_Size@0_0},
    size_of(_witnesses_where_size_of@1_0, <<"abcde"/utf8>>).

local_to_celsius(_n@0_0) ->
    _witness_From@1_0 =
        'Elixir.Patina.ForeignImpl.$impl.Temperatures.Celsius':'temperatures::From<foreign_impl::LocalDegrees>::$patina_witness'(),
    _witnesses_where_to_celsius@2_0 = {_witness_From@1_0},
    'Elixir.Patina.Temperatures':to_celsius(_witnesses_where_to_celsius@2_0,
                                            _n@0_0).

fahrenheit_to_celsius(_f@0_0) ->
    _witness_From@1_0 =
        'Elixir.Patina.Temperatures.Celsius':'temperatures::From<temperatures::Fahrenheit>::$patina_witness'(),
    _witnesses_where_to_celsius@2_0 = {_witness_From@1_0},
    'Elixir.Patina.Temperatures':to_celsius(_witnesses_where_to_celsius@2_0,
                                            _f@0_0).

kelvin_to_celsius(_k@0_0) ->
    _witness_From@1_0 =
        'Elixir.Patina.MoreTemperatures.$impl.Temperatures.Celsius':'temperatures::From<more_temperatures::Kelvin>::$patina_witness'(),
    _witnesses_where_to_celsius@2_0 = {_witness_From@1_0},
    'Elixir.Patina.Temperatures':to_celsius(_witnesses_where_to_celsius@2_0,
                                            _k@0_0).

rankine_to_celsius(_r@0_0) ->
    _witness_From@1_0 =
        'Elixir.Patina.MoreTemperatures.$impl.Temperatures.Celsius':'temperatures::From<more_temperatures::Rankine>::$patina_witness'(),
    _witnesses_where_to_celsius@2_0 = {_witness_From@1_0},
    'Elixir.Patina.Temperatures':to_celsius(_witnesses_where_to_celsius@2_0,
                                            _r@0_0).

label_shout() ->
    _witness_Shout@0_0 =
        'Elixir.Patina.LabelShout.$impl.Traits.Label':'label_shout::Shout::$patina_witness'(),
    _witnesses_where_shout@1_0 = {_witness_Shout@0_0},
    'Elixir.Patina.LabelShout':shout(_witnesses_where_shout@1_0,
                                     <<"x"/utf8>>).

label_whisper() ->
    _witness_Whisper@0_0 =
        'Elixir.Patina.LabelWhisper.$impl.Traits.Label':'label_whisper::Whisper::$patina_witness'(),
    _witnesses_where_whisper@1_0 = {_witness_Whisper@0_0},
    'Elixir.Patina.LabelWhisper':whisper(_witnesses_where_whisper@1_0,
                                         <<"x"/utf8>>).

label_show() ->
    _witness_Show@0_0 =
        'Elixir.Patina.Traits.Label':'traits::Show::$patina_witness'(),
    'Elixir.Patina.Traits.Label':'traits::Show::show'(_witness_Show@0_0,
                                                      <<"shown"/utf8>>).

loud_generic(_witnesses_where_loud_generic@0_0,
             _t@1_0) ->
    _witness_Loud@2_0 = erlang:element(1,
                                       _witnesses_where_loud_generic@0_0),
    _vtable_Loud@3_0 = erlang:element(1, _witness_Loud@2_0),
    _fn_Loud_loud@4_0 = erlang:element(1, _vtable_Loud@3_0),
    _fn_Loud_loud@4_0(_witness_Loud@2_0, _t@1_0).

blanket_on_foreign_type(_s@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.Traits.Label':'traits::Show::$patina_witness'(),
    _witnesses_where_Loud@2_0 = {_witness_Show@1_0},
    _witness_Loud@3_0 =
        'Elixir.Patina.Traits.$impl':'traits::Loud::$patina_witness'(_witnesses_where_Loud@2_0),
    'Elixir.Patina.Traits.$impl':'traits::Loud::loud'(_witness_Loud@3_0,
                                                      _s@0_0).

blanket_on_local_type() ->
    _witness_Show@0_0 =
        'Elixir.Patina.ForeignImpl.Local':'traits::Show::$patina_witness'(),
    _witnesses_where_Loud@1_0 = {_witness_Show@0_0},
    _witness_Loud@2_0 =
        'Elixir.Patina.Traits.$impl':'traits::Loud::$patina_witness'(_witnesses_where_Loud@1_0),
    'Elixir.Patina.Traits.$impl':'traits::Loud::loud'(_witness_Loud@2_0,
                                                      'Elixir.Patina.ForeignImpl.Local').

blanket_generic_both(_s@0_0) ->
    _witness_Show@1_0 =
        'Elixir.Patina.Traits.Label':'traits::Show::$patina_witness'(),
    _witnesses_where_Loud@2_0 = {_witness_Show@1_0},
    _witness_Loud@3_0 =
        'Elixir.Patina.Traits.$impl':'traits::Loud::$patina_witness'(_witnesses_where_Loud@2_0),
    _witnesses_where_loud_generic@4_0 = {_witness_Loud@3_0},
    _witness_Show@5_0 =
        'Elixir.Patina.ForeignImpl.Local':'traits::Show::$patina_witness'(),
    _witnesses_where_Loud@6_0 = {_witness_Show@5_0},
    _witness_Loud@7_0 =
        'Elixir.Patina.Traits.$impl':'traits::Loud::$patina_witness'(_witnesses_where_Loud@6_0),
    _witnesses_where_loud_generic@8_0 = {_witness_Loud@7_0},
    <<(loud_generic(_witnesses_where_loud_generic@4_0,
                    _s@0_0))/binary,
      (loud_generic(_witnesses_where_loud_generic@8_0,
                    'Elixir.Patina.ForeignImpl.Local'))/binary>>.
