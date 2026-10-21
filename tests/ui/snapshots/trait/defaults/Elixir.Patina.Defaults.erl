-file("tests/ui/src/trait/defaults.pt", 1).

-module('Elixir.Patina.Defaults').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'Defaults::Animal::calls_required'/2,
         'Defaults::Animal::calls_default'/2,
         'Defaults::Animal::with_bounded_param'/4,
         'Defaults::Animal::returns_self'/2,
         'Defaults::Animal::whisper'/2,
         sheep_calls_required/1,
         dog_calls_required/1,
         sheep_with_bounded_param/1,
         dog_with_bounded_param/1,
         sheep_calls_stubs_locally/1,
         sheep_via/1,
         dog_via/1,
         sheep_returns_self/1,
         dog_returns_self/1,
         'Defaults::Combine::uses_rhs_witness'/4,
         combine_coin_piles/2,
         combine_coin_with_note/2,
         combine_note_piles_via/2,
         combine_coin_with_note_via/2,
         double/1,
         'Defaults::Greeter::greet'/2,
         greet/1,
         greet_through_witness/1,
         'Defaults::Container::needs_item_witness'/3,
         describe_boxed/1,
         describe_wrapped/1,
         describe_rewrapped/1,
         describe_loose/1,
         describe_first_loose/1,
         describe_nested/1,
         'Defaults::Maker::make'/2,
         make/0,
         'Defaults::Numbered::$patina_const::D'/1,
         'Defaults::Numbered::consts'/2,
         one_consts/0,
         two_consts/0,
         plus_one_consts/0,
         sum_two/0,
         one_c/0,
         plus_two_d/0,
         doubled_k/0,
         int_ne/2,
         str_ne/2,
         p_ne/2,
         p_differ/2,
         int_differ/2,
         ring_loud/0,
         ring_quiet/0,
         ring_quiet_directly/0,
         describe_through_pointer/1,
         ne_through_pointer/2,
         sheep_calls_required_through_pointer/1]).

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

'Defaults::Animal::calls_required'(_witness_Animal@0_0,
                                   _self@1_0) ->
    _vtable_Animal@2_0 = erlang:element(1,
                                        _witness_Animal@0_0),
    _fn_Animal_noise@3_0 = erlang:element(2,
                                          _vtable_Animal@2_0),
    _fn_Animal_name@4_0 = erlang:element(1,
                                         _vtable_Animal@2_0),
    begin
        begin
            _element@5_0 = _fn_Animal_name@4_0(_witness_Animal@0_0,
                                               _self@1_0),
            _element@6_0 = _fn_Animal_noise@3_0(_witness_Animal@0_0,
                                                _self@1_0),
            {_element@5_0, _element@6_0}
        end
    end.

'Defaults::Animal::calls_default'(_witness_Animal@0_0,
                                  _self@1_0) ->
    _vtable_Animal@2_0 = erlang:element(1,
                                        _witness_Animal@0_0),
    _fn_Animal_calls_required@3_0 = erlang:element(3,
                                                   _vtable_Animal@2_0),
    begin
        begin
            _element@4_0 =
                _fn_Animal_calls_required@3_0(_witness_Animal@0_0,
                                              _self@1_0),
            _element@5_0 =
                _fn_Animal_calls_required@3_0(_witness_Animal@0_0,
                                              _self@1_0),
            {_element@4_0, _element@5_0}
        end
    end.

'Defaults::Animal::with_bounded_param'(_witness_Animal@0_0,
                                       _witnesses_where_Animal_with_bounded_param@1_0,
                                       _self@2_0, _tag@3_0) ->
    _vtable_Animal@4_0 = erlang:element(1,
                                        _witness_Animal@0_0),
    _fn_Animal_noise@5_0 = erlang:element(2,
                                          _vtable_Animal@4_0),
    _fn_Animal_name@6_0 = erlang:element(1,
                                         _vtable_Animal@4_0),
    _witness_Tag@7_0 = erlang:element(1,
                                      _witnesses_where_Animal_with_bounded_param@1_0),
    _vtable_Tag@8_0 = erlang:element(1, _witness_Tag@7_0),
    _fn_Tag_tag@9_0 = erlang:element(1, _vtable_Tag@8_0),
    begin
        begin
            _element@10_0 = _fn_Animal_name@6_0(_witness_Animal@0_0,
                                                _self@2_0),
            _element@11_0 =
                _fn_Animal_noise@5_0(_witness_Animal@0_0, _self@2_0),
            _element@12_0 = _fn_Tag_tag@9_0(_witness_Tag@7_0,
                                            _tag@3_0),
            {_element@10_0, _element@11_0, _element@12_0}
        end
    end.

'Defaults::Animal::returns_self'(_witness_Animal@0_0,
                                 _self@1_0) ->
    _this@2_0 = _self@1_0,
    _this@2_0.

'Defaults::Animal::whisper'(_witness_Animal@0_0,
                            _self@1_0) ->
    _vtable_Animal@2_0 = erlang:element(1,
                                        _witness_Animal@0_0),
    _fn_Animal_name@3_0 = erlang:element(1,
                                         _vtable_Animal@2_0),
    begin
        _fn_Animal_name@3_0(_witness_Animal@0_0, _self@1_0)
    end.

via(_witnesses_where_via@0_0, _animal@1_0) ->
    _witness_Animal@2_0 = erlang:element(1,
                                         _witnesses_where_via@0_0),
    _vtable_Animal@3_0 = erlang:element(1,
                                        _witness_Animal@2_0),
    _fn_Animal_calls_default@4_0 = erlang:element(4,
                                                  _vtable_Animal@3_0),
    _fn_Animal_calls_required@5_0 = erlang:element(3,
                                                   _vtable_Animal@3_0),
    begin
        begin
            _element@6_0 =
                _fn_Animal_calls_required@5_0(_witness_Animal@2_0,
                                              _animal@1_0),
            _element@7_0 =
                _fn_Animal_calls_default@4_0(_witness_Animal@2_0,
                                             _animal@1_0),
            {_element@6_0, _element@7_0}
        end
    end.

sheep_calls_required(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
    begin
        'Defaults::Animal::calls_required'(_witness_Animal@1_0,
                                           _name@0_0)
    end.

dog_calls_required(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
    begin
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::calls_required'(_witness_Animal@1_0,
                                                                        _name@0_0)
    end.

sheep_with_bounded_param(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
    _witness_Tag@2_0 =
        'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'(),
    _witnesses_where_Animal_with_bounded_param@3_0 =
        {_witness_Tag@2_0},
    begin
        'Defaults::Animal::with_bounded_param'(_witness_Animal@1_0,
                                               _witnesses_where_Animal_with_bounded_param@3_0,
                                               _name@0_0,
                                               'Elixir.Patina.Defaults.Bell')
    end.

dog_with_bounded_param(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
    _witness_Tag@2_0 =
        'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'(),
    _witnesses_where_Animal_with_bounded_param@3_0 =
        {_witness_Tag@2_0},
    begin
        'Defaults::Animal::with_bounded_param'(_witness_Animal@1_0,
                                               _witnesses_where_Animal_with_bounded_param@3_0,
                                               _name@0_0,
                                               'Elixir.Patina.Defaults.Bell')
    end.

sheep_calls_stubs_locally(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':calls_stubs_locally(_name@0_0).

sheep_via(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
    _witnesses_where_via@2_0 = {_witness_Animal@1_0},
    begin via(_witnesses_where_via@2_0, _name@0_0) end.

dog_via(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
    _witnesses_where_via@2_0 = {_witness_Animal@1_0},
    begin via(_witnesses_where_via@2_0, _name@0_0) end.

sheep_returns_self(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
    begin
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::name'(_witness_Animal@1_0,
                                                                'Defaults::Animal::returns_self'(_witness_Animal@1_0,
                                                                                                 _name@0_0))
    end.

dog_returns_self(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
    begin
        'Elixir.Patina.Defaults.Dog':'Defaults::Animal::name'(_witness_Animal@1_0,
                                                              'Defaults::Animal::returns_self'(_witness_Animal@1_0,
                                                                                               _name@0_0))
    end.

'Defaults::Combine::uses_rhs_witness'(_witness_Combine@0_0,
                                      _witnesses_where_Combine_uses_rhs_witness@1_0,
                                      _self@2_0, _other@3_0) ->
    _supertraits_Combine@4_0 = erlang:element(2,
                                              _witness_Combine@0_0),
    _witness_Value@5_0 = erlang:element(1,
                                        _supertraits_Combine@4_0),
    _vtable_Value@6_0 = erlang:element(1,
                                       _witness_Value@5_0),
    _fn_Value_value@7_0 = erlang:element(1,
                                         _vtable_Value@6_0),
    _witness_Value@8_0 = erlang:element(1,
                                        _witnesses_where_Combine_uses_rhs_witness@1_0),
    _vtable_Value@9_0 = erlang:element(1,
                                       _witness_Value@8_0),
    _fn_Value_value@10_0 = erlang:element(1,
                                          _vtable_Value@9_0),
    begin
        _fn_Value_value@7_0(_witness_Value@5_0, _self@2_0) +
            _fn_Value_value@10_0(_witness_Value@8_0, _other@3_0)
    end.

combine_via(_witnesses_where_combine_via@0_0, _a@1_0,
            _b@2_0) ->
    _witness_Value@3_0 = erlang:element(2,
                                        _witnesses_where_combine_via@0_0),
    _witness_Combine@4_0 = erlang:element(1,
                                          _witnesses_where_combine_via@0_0),
    _vtable_Combine@5_0 = erlang:element(1,
                                         _witness_Combine@4_0),
    _fn_Combine_uses_rhs_witness@6_0 = erlang:element(1,
                                                      _vtable_Combine@5_0),
    _witnesses_where_Combine_uses_rhs_witness@7_0 =
        {_witness_Value@3_0},
    begin
        _fn_Combine_uses_rhs_witness@6_0(_witness_Combine@4_0,
                                         _witnesses_where_Combine_uses_rhs_witness@7_0,
                                         _a@1_0,
                                         _b@2_0)
    end.

combine_coin_piles(_a@0_0, _b@1_0) ->
    _witness_Value@2_0 =
        'Elixir.Patina.Defaults.Coin':'Defaults::Value::$patina_witness'(),
    _witnesses_where_Combine@3_0 = {_witness_Value@2_0},
    _witness_Combine@4_0 =
        'Elixir.Patina.Defaults.Pile':'Defaults::Combine::$patina_witness'(_witnesses_where_Combine@3_0),
    _supertraits_Combine@5_0 = erlang:element(2,
                                              _witness_Combine@4_0),
    _witness_Value@6_0 = erlang:element(1,
                                        _supertraits_Combine@5_0),
    _witnesses_where_Combine_uses_rhs_witness@7_0 =
        {_witness_Value@6_0},
    begin
        'Defaults::Combine::uses_rhs_witness'(_witness_Combine@4_0,
                                              _witnesses_where_Combine_uses_rhs_witness@7_0,
                                              begin _0@8_0 = _a@0_0, _0@8_0 end,
                                              begin _0@9_0 = _b@1_0, _0@9_0 end)
    end.

combine_coin_with_note(_a@0_0, _b@1_0) ->
    _witness_Combine@2_0 =
        'Elixir.Patina.Defaults.Coin':'Defaults::Combine<Defaults::Note>::$patina_witness'(),
    _witness_Value@3_0 =
        'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'(),
    _witnesses_where_Combine_uses_rhs_witness@4_0 =
        {_witness_Value@3_0},
    begin
        'Defaults::Combine::uses_rhs_witness'(_witness_Combine@2_0,
                                              _witnesses_where_Combine_uses_rhs_witness@4_0,
                                              _a@0_0,
                                              _b@1_0)
    end.

combine_note_piles_via(_a@0_0, _b@1_0) ->
    _witness_Value@2_0 =
        'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'(),
    _witnesses_where_Combine@3_0 = {_witness_Value@2_0},
    _witness_Combine@4_0 =
        'Elixir.Patina.Defaults.Pile':'Defaults::Combine::$patina_witness'(_witnesses_where_Combine@3_0),
    _supertraits_Combine@5_0 = erlang:element(2,
                                              _witness_Combine@4_0),
    _witness_Value@6_0 = erlang:element(1,
                                        _supertraits_Combine@5_0),
    _witnesses_where_combine_via@7_0 =
        {_witness_Combine@4_0, _witness_Value@6_0},
    begin
        combine_via(_witnesses_where_combine_via@7_0,
                    begin _0@8_0 = _a@0_0, _0@8_0 end,
                    begin _0@9_0 = _b@1_0, _0@9_0 end)
    end.

combine_coin_with_note_via(_a@0_0, _b@1_0) ->
    _witness_Combine@2_0 =
        'Elixir.Patina.Defaults.Coin':'Defaults::Combine<Defaults::Note>::$patina_witness'(),
    _witness_Value@3_0 =
        'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'(),
    _witnesses_where_combine_via@4_0 =
        {_witness_Combine@2_0, _witness_Value@3_0},
    begin
        combine_via(_witnesses_where_combine_via@4_0,
                    _a@0_0,
                    _b@1_0)
    end.

double(_x@0_0) ->
    _witness_Doubler@1_0 =
        'Elixir.Patina.Defaults.N':'Defaults::Inner::Doubler::$patina_witness'(),
    begin
        'Elixir.Patina.Defaults.Inner':'Defaults::Inner::Doubler::doubled'(_witness_Doubler@1_0,
                                                                           _x@0_0)
    end.

'Defaults::Greeter::greet'(_witness_Greeter@0_0,
                           _self@1_0) ->
    _supertraits_Greeter@2_0 = erlang:element(2,
                                              _witness_Greeter@0_0),
    _witness_Named@3_0 = erlang:element(1,
                                        _supertraits_Greeter@2_0),
    _vtable_Named@4_0 = erlang:element(1,
                                       _witness_Named@3_0),
    _fn_Named_name@5_0 = erlang:element(1,
                                        _vtable_Named@4_0),
    begin
        begin
            _element@6_0 = _fn_Named_name@5_0(_witness_Named@3_0,
                                              _self@1_0),
            {<<"Hello"/utf8>>, _element@6_0}
        end
    end.

greet_via(_witnesses_where_greet_via@0_0,
          _greeter@1_0) ->
    _witness_Greeter@2_0 = erlang:element(1,
                                          _witnesses_where_greet_via@0_0),
    _vtable_Greeter@3_0 = erlang:element(1,
                                         _witness_Greeter@2_0),
    _fn_Greeter_greet@4_0 = erlang:element(1,
                                           _vtable_Greeter@3_0),
    begin
        _fn_Greeter_greet@4_0(_witness_Greeter@2_0,
                              _greeter@1_0)
    end.

greet(_name@0_0) ->
    _witness_Greeter@1_0 =
        'Elixir.Patina.Defaults.Person':'Defaults::Greeter::$patina_witness'(),
    begin
        'Defaults::Greeter::greet'(_witness_Greeter@1_0,
                                   _name@0_0)
    end.

greet_through_witness(_name@0_0) ->
    _witness_Greeter@1_0 =
        'Elixir.Patina.Defaults.Person':'Defaults::Greeter::$patina_witness'(),
    _witnesses_where_greet_via@2_0 = {_witness_Greeter@1_0},
    begin
        greet_via(_witnesses_where_greet_via@2_0, _name@0_0)
    end.

'Defaults::Container::needs_item_witness'(_witness_Container@0_0,
                                          _witnesses_where_Container_needs_item_witness@1_0,
                                          _self@2_0) ->
    _vtable_Container@3_0 = erlang:element(1,
                                           _witness_Container@0_0),
    _fn_Container_get@4_0 = erlang:element(2,
                                           _vtable_Container@3_0),
    _witness_Describe@5_0 = erlang:element(1,
                                           _witnesses_where_Container_needs_item_witness@1_0),
    _vtable_Describe@6_0 = erlang:element(1,
                                          _witness_Describe@5_0),
    _fn_Describe_describe@7_0 = erlang:element(1,
                                               _vtable_Describe@6_0),
    begin
        _fn_Describe_describe@7_0(_witness_Describe@5_0,
                                  _fn_Container_get@4_0(_witness_Container@0_0,
                                                        _self@2_0))
    end.

first(_witnesses_where_first@0_0, _container@1_0) ->
    _witness_Describe@2_0 = erlang:element(2,
                                           _witnesses_where_first@0_0),
    _witness_Container@3_0 = erlang:element(1,
                                            _witnesses_where_first@0_0),
    _vtable_Describe@4_0 = erlang:element(1,
                                          _witness_Describe@2_0),
    _fn_Describe_describe@5_0 = erlang:element(1,
                                               _vtable_Describe@4_0),
    _vtable_Container@6_0 = erlang:element(1,
                                           _witness_Container@3_0),
    _fn_Container_get@7_0 = erlang:element(2,
                                           _vtable_Container@6_0),
    begin
        _fn_Describe_describe@5_0(_witness_Describe@2_0,
                                  _fn_Container_get@7_0(_witness_Container@3_0,
                                                        _container@1_0))
    end.

nested(_witnesses_where_nested@0_0, _container@1_0) ->
    _witness_Describe@2_0 = erlang:element(3,
                                           _witnesses_where_nested@0_0),
    _witness_Container@3_0 = erlang:element(2,
                                            _witnesses_where_nested@0_0),
    _witness_Container@4_0 = erlang:element(1,
                                            _witnesses_where_nested@0_0),
    _vtable_Describe@5_0 = erlang:element(1,
                                          _witness_Describe@2_0),
    _fn_Describe_describe@6_0 = erlang:element(1,
                                               _vtable_Describe@5_0),
    _vtable_Container@7_0 = erlang:element(1,
                                           _witness_Container@3_0),
    _fn_Container_get@8_0 = erlang:element(2,
                                           _vtable_Container@7_0),
    _vtable_Container@9_0 = erlang:element(1,
                                           _witness_Container@4_0),
    _fn_Container_get@10_0 = erlang:element(2,
                                            _vtable_Container@9_0),
    begin
        _fn_Describe_describe@6_0(_witness_Describe@2_0,
                                  _fn_Container_get@8_0(_witness_Container@3_0,
                                                        _fn_Container_get@10_0(_witness_Container@4_0,
                                                                               _container@1_0)))
    end.

describe_boxed(_name@0_0) ->
    _witness_Container@1_0 =
        'Elixir.Patina.Defaults.Boxed':'Defaults::Container::$patina_witness'(),
    _witness_Describe@2_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_Container_needs_item_witness@3_0 =
        {_witness_Describe@2_0},
    begin
        'Defaults::Container::needs_item_witness'(_witness_Container@1_0,
                                                  _witnesses_where_Container_needs_item_witness@3_0,
                                                  begin
                                                      _0@4_0 = _name@0_0, _0@4_0
                                                  end)
    end.

describe_wrapped(_name@0_0) ->
    _witness_Describe@1_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_Container@2_0 =
        {_witness_Describe@1_0},
    _witness_Container@3_0 =
        'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::$patina_witness'(_witnesses_where_Container@2_0),
    begin
        'Defaults::Container::needs_item_witness'(_witness_Container@3_0,
                                                  _witnesses_where_Container@2_0,
                                                  begin
                                                      _0@4_0 = _name@0_0, _0@4_0
                                                  end)
    end.

describe_rewrapped(_name@0_0) ->
    _witness_Container@1_0 =
        'Elixir.Patina.Defaults.Rewrapped':'Defaults::Container::$patina_witness'(),
    _witness_Describe@2_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_Container_needs_item_witness@3_0 =
        {_witness_Describe@2_0},
    begin
        'Defaults::Container::needs_item_witness'(_witness_Container@1_0,
                                                  _witnesses_where_Container_needs_item_witness@3_0,
                                                  begin
                                                      _0@5_0 = begin
                                                                   _0@4_0 =
                                                                       _name@0_0,
                                                                   _0@4_0
                                                               end,
                                                      _0@5_0
                                                  end)
    end.

describe_loose(_name@0_0) ->
    _witness_Container@1_0 =
        'Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
    _witness_Describe@2_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_Container_needs_item_witness@3_0 =
        {_witness_Describe@2_0},
    begin
        'Defaults::Container::needs_item_witness'(_witness_Container@1_0,
                                                  _witnesses_where_Container_needs_item_witness@3_0,
                                                  begin
                                                      _0@4_0 = _name@0_0, _0@4_0
                                                  end)
    end.

describe_first_loose(_name@0_0) ->
    _witness_Container@1_0 =
        'Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
    _witness_Describe@2_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_first@3_0 = {_witness_Container@1_0,
                                  _witness_Describe@2_0},
    begin
        first(_witnesses_where_first@3_0,
              begin _0@4_0 = _name@0_0, _0@4_0 end)
    end.

describe_nested(_name@0_0) ->
    _witness_Describe@1_0 =
        'Elixir.Patina.Defaults.Boxed':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_Container@2_0 =
        {_witness_Describe@1_0},
    _witness_Container@3_0 =
        'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::$patina_witness'(_witnesses_where_Container@2_0),
    _witness_Container@4_0 =
        'Elixir.Patina.Defaults.Boxed':'Defaults::Container::$patina_witness'(),
    _witness_Describe@5_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_nested@6_0 = {_witness_Container@3_0,
                                   _witness_Container@4_0,
                                   _witness_Describe@5_0},
    begin
        nested(_witnesses_where_nested@6_0,
               begin
                   _0@8_0 = begin _0@7_0 = _name@0_0, _0@7_0 end, _0@8_0
               end)
    end.

'Defaults::Maker::make'(_witness_Maker@0_0,
                        _self@1_0) ->
    <<"made"/utf8>>.

make() ->
    _witness_Maker@0_0 =
        'Elixir.Patina.Defaults.Boxed':'Defaults::Maker::$patina_witness'(),
    begin
        _ = 'Defaults::Maker::make'(_witness_Maker@0_0,
                                    begin
                                        _0@1_0 = <<"unused"/utf8>>, _0@1_0
                                    end),
        {}
    end.

'Defaults::Numbered::$patina_const::D'(_witness_Numbered@0_0) ->
    100.

'Defaults::Numbered::consts'(_witness_Numbered@0_0,
                             _self@1_0) ->
    _vtable_Numbered@2_0 = erlang:element(1,
                                          _witness_Numbered@0_0),
    _const_fn_Numbered_D@3_0 = erlang:element(2,
                                              _vtable_Numbered@2_0),
    _const_fn_Numbered_C@4_0 = erlang:element(1,
                                              _vtable_Numbered@2_0),
    _const_Numbered_C@5_0 =
        _const_fn_Numbered_C@4_0(_witness_Numbered@0_0),
    _const_Numbered_D@6_0 =
        _const_fn_Numbered_D@3_0(_witness_Numbered@0_0),
    begin
        begin
            _element@7_0 = _const_Numbered_C@5_0,
            _element@8_0 = _const_Numbered_D@6_0,
            {_element@7_0, _element@8_0}
        end
    end.

sum(_witnesses_where_sum@0_0, __t@1_0) ->
    _witness_Numbered@2_0 = erlang:element(1,
                                           _witnesses_where_sum@0_0),
    _vtable_Numbered@3_0 = erlang:element(1,
                                          _witness_Numbered@2_0),
    _const_fn_Numbered_D@4_0 = erlang:element(2,
                                              _vtable_Numbered@3_0),
    _const_fn_Numbered_C@5_0 = erlang:element(1,
                                              _vtable_Numbered@3_0),
    _const_Numbered_C@6_0 =
        _const_fn_Numbered_C@5_0(_witness_Numbered@2_0),
    _const_Numbered_D@7_0 =
        _const_fn_Numbered_D@4_0(_witness_Numbered@2_0),
    begin _const_Numbered_C@6_0 + _const_Numbered_D@7_0 end.

one_consts() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'(),
    begin
        'Defaults::Numbered::consts'(_witness_Numbered@0_0,
                                     'Elixir.Patina.Defaults.One')
    end.

two_consts() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'(),
    begin
        'Defaults::Numbered::consts'(_witness_Numbered@0_0,
                                     'Elixir.Patina.Defaults.Two')
    end.

plus_one_consts() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'(),
    _witnesses_where_Numbered@1_0 = {_witness_Numbered@0_0},
    _witness_Numbered@2_0 =
        'Elixir.Patina.Defaults.Plus':'Defaults::Numbered::$patina_witness'(_witnesses_where_Numbered@1_0),
    begin
        'Defaults::Numbered::consts'(_witness_Numbered@2_0,
                                     begin
                                         _0@3_0 = 'Elixir.Patina.Defaults.One',
                                         _0@3_0
                                     end)
    end.

sum_two() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'(),
    _witnesses_where_sum@1_0 = {_witness_Numbered@0_0},
    begin
        sum(_witnesses_where_sum@1_0,
            'Elixir.Patina.Defaults.Two')
    end.

one_c() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'(),
    _const_Numbered_C@1_0 =
        'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_const::C'(_witness_Numbered@0_0),
    begin _const_Numbered_C@1_0 end.

plus_two_d() ->
    _witness_Numbered@0_0 =
        'Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'(),
    _witnesses_where_Numbered@1_0 = {_witness_Numbered@0_0},
    _witness_Numbered@2_0 =
        'Elixir.Patina.Defaults.Plus':'Defaults::Numbered::$patina_witness'(_witnesses_where_Numbered@1_0),
    _const_Numbered_D@3_0 =
        'Defaults::Numbered::$patina_const::D'(_witness_Numbered@2_0),
    begin _const_Numbered_D@3_0 end.

doubled_k() ->
    begin
        _witness_Konst@0_0 =
            'Elixir.Patina.Defaults.Holder':'Defaults::Konst::$patina_witness'(),
        _const_Konst_K@1_0 =
            'Elixir.Patina.Defaults.Holder':'Defaults::Konst::$patina_const::K'(_witness_Konst@0_0),
        _const_Konst_K@1_0 * 2
    end.

differ(_witnesses_where_differ@0_0, _a@1_0, _b@2_0) ->
    _witness_PartialEq@3_0 = erlang:element(1,
                                            _witnesses_where_differ@0_0),
    _vtable_PartialEq@4_0 = erlang:element(1,
                                           _witness_PartialEq@3_0),
    _fn_PartialEq_ne@5_0 = erlang:element(2,
                                          _vtable_PartialEq@4_0),
    begin
        _fn_PartialEq_ne@5_0(_witness_PartialEq@3_0,
                             _a@1_0,
                             _b@2_0)
    end.

int_ne(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Int':'Core::PartialEq::$patina_witness'(),
    begin
        'Elixir.Patina.Core.Int':'Core::PartialEq::ne'(_witness_PartialEq@2_0,
                                                       _a@0_0,
                                                       _b@1_0)
    end.

str_ne(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Str':'Core::PartialEq::$patina_witness'(),
    begin
        'Elixir.Patina.Core.Str':'Core::PartialEq::ne'(_witness_PartialEq@2_0,
                                                       _a@0_0,
                                                       _b@1_0)
    end.

p_ne(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'(),
    begin
        'Elixir.Patina.Core':'Core::PartialEq::ne'(_witness_PartialEq@2_0,
                                                   _a@0_0,
                                                   _b@1_0)
    end.

p_differ(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'(),
    _witnesses_where_differ@3_0 = {_witness_PartialEq@2_0},
    begin
        differ(_witnesses_where_differ@3_0, _a@0_0, _b@1_0)
    end.

int_differ(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Core.Int':'Core::PartialEq::$patina_witness'(),
    _witnesses_where_differ@3_0 = {_witness_PartialEq@2_0},
    begin
        differ(_witnesses_where_differ@3_0, _a@0_0, _b@1_0)
    end.

ring_via(_witnesses_where_ring_via@0_0, _ringer@1_0) ->
    _witness_Ring@2_0 = erlang:element(1,
                                       _witnesses_where_ring_via@0_0),
    _vtable_Ring@3_0 = erlang:element(1, _witness_Ring@2_0),
    _fn_Ring_ring@4_0 = erlang:element(1, _vtable_Ring@3_0),
    _witness_Tag@5_0 =
        'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'(),
    _witnesses_where_Ring_ring@6_0 = {_witness_Tag@5_0},
    begin
        _fn_Ring_ring@4_0(_witness_Ring@2_0,
                          _witnesses_where_Ring_ring@6_0,
                          _ringer@1_0,
                          'Elixir.Patina.Defaults.Bell')
    end.

ring_loud() ->
    _witness_Ring@0_0 =
        'Elixir.Patina.Defaults.Loud':'Defaults::Ring::$patina_witness'(),
    _witnesses_where_ring_via@1_0 = {_witness_Ring@0_0},
    begin
        ring_via(_witnesses_where_ring_via@1_0,
                 'Elixir.Patina.Defaults.Loud')
    end.

ring_quiet() ->
    _witness_Ring@0_0 =
        'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::$patina_witness'(),
    _witnesses_where_ring_via@1_0 = {_witness_Ring@0_0},
    begin
        ring_via(_witnesses_where_ring_via@1_0,
                 'Elixir.Patina.Defaults.Quiet')
    end.

ring_quiet_directly() ->
    _witness_Ring@0_0 =
        'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::$patina_witness'(),
    _witness_Tag@1_0 =
        'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'(),
    _witnesses_where_Ring_ring@2_0 = {_witness_Tag@1_0},
    begin
        'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::ring'(_witness_Ring@0_0,
                                                              _witnesses_where_Ring_ring@2_0,
                                                              'Elixir.Patina.Defaults.Quiet',
                                                              'Elixir.Patina.Defaults.Bell')
    end.

calls_required_through_pointer(_witnesses_where_calls_required_through_pointer@0_0,
                               _animal@1_0) ->
    _witness_Animal@2_0 = erlang:element(1,
                                         _witnesses_where_calls_required_through_pointer@0_0),
    _vtable_Animal@3_0 = erlang:element(1,
                                        _witness_Animal@2_0),
    _fn_Animal_calls_required@4_0 = erlang:element(3,
                                                   _vtable_Animal@3_0),
    begin
        _calls_required@6_0 = fun (_arg@5_0) ->
                                      _fn_Animal_calls_required@4_0(_witness_Animal@2_0,
                                                                    _arg@5_0)
                              end,
        _calls_required@6_0(_animal@1_0)
    end.

describe_through_pointer(_name@0_0) ->
    _witness_Container@1_0 =
        'Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
    _witness_Describe@2_0 =
        'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'(),
    _witnesses_where_first@3_0 = {_witness_Container@1_0,
                                  _witness_Describe@2_0},
    begin
        _describe@5_0 = fun (_arg@4_0) ->
                                first(_witnesses_where_first@3_0, _arg@4_0)
                        end,
        _describe@5_0(begin _0@6_0 = _name@0_0, _0@6_0 end)
    end.

ne_through_pointer(_a@0_0, _b@1_0) ->
    _witness_PartialEq@2_0 =
        'Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'(),
    begin
        _ne@5_0 = fun (_arg@3_0, _arg@4_0) ->
                          'Elixir.Patina.Core':'Core::PartialEq::ne'(_witness_PartialEq@2_0,
                                                                     _arg@3_0,
                                                                     _arg@4_0)
                  end,
        _ne@5_0(_a@0_0, _b@1_0)
    end.

sheep_calls_required_through_pointer(_name@0_0) ->
    _witness_Animal@1_0 =
        'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
    _witnesses_where_calls_required_through_pointer@2_0 =
        {_witness_Animal@1_0},
    begin
        calls_required_through_pointer(_witnesses_where_calls_required_through_pointer@2_0,
                                       _name@0_0)
    end.
