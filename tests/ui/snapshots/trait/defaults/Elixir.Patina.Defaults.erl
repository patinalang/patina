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

'Defaults::Animal::calls_required'(_witness@0_0,
                                   _self@1_0) ->
    begin
        _element@2_0 = (erlang:element(1,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@1_0),
        _element@3_0 = (erlang:element(2,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@1_0),
        {_element@2_0, _element@3_0}
    end.

'Defaults::Animal::calls_default'(_witness@0_0,
                                  _self@1_0) ->
    begin
        _element@2_0 = (erlang:element(3,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@1_0),
        _element@3_0 = (erlang:element(3,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@1_0),
        {_element@2_0, _element@3_0}
    end.

'Defaults::Animal::with_bounded_param'(_witness@0_0,
                                       _witness@1_0, _self@2_0, _tag@3_0) ->
    begin
        _element@4_0 = (erlang:element(1,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@2_0),
        _element@5_0 = (erlang:element(2,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0,
                                                                      _self@2_0),
        _element@7_0 = begin
                           _witness@6_0 = erlang:element(1, _witness@1_0),
                           (erlang:element(1,
                                           erlang:element(1,
                                                          _witness@6_0)))(_witness@6_0,
                                                                          _tag@3_0)
                       end,
        {_element@4_0, _element@5_0, _element@7_0}
    end.

'Defaults::Animal::returns_self'(_witness@0_0,
                                 _self@1_0) ->
    _this@2_0 = _self@1_0,
    _this@2_0.

'Defaults::Animal::whisper'(_witness@0_0, _self@1_0) ->
    (erlang:element(1,
                    erlang:element(1, _witness@0_0)))(_witness@0_0,
                                                      _self@1_0).

via(_witness@0_0, _animal@1_0) ->
    begin
        _element@3_0 = begin
                           _witness@2_0 = erlang:element(1, _witness@0_0),
                           (erlang:element(3,
                                           erlang:element(1,
                                                          _witness@2_0)))(_witness@2_0,
                                                                          _animal@1_0)
                       end,
        _element@5_0 = begin
                           _witness@4_0 = erlang:element(1, _witness@0_0),
                           (erlang:element(4,
                                           erlang:element(1,
                                                          _witness@4_0)))(_witness@4_0,
                                                                          _animal@1_0)
                       end,
        {_element@3_0, _element@5_0}
    end.

sheep_calls_required(_name@0_0) ->
    'Defaults::Animal::calls_required'('Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
                                       _name@0_0).

dog_calls_required(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::calls_required'('Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
                                                                    _name@0_0).

sheep_with_bounded_param(_name@0_0) ->
    'Defaults::Animal::with_bounded_param'('Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
                                           {'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'()},
                                           _name@0_0,
                                           'Elixir.Patina.Defaults.Bell').

dog_with_bounded_param(_name@0_0) ->
    'Defaults::Animal::with_bounded_param'('Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
                                           {'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'()},
                                           _name@0_0,
                                           'Elixir.Patina.Defaults.Bell').

sheep_calls_stubs_locally(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':calls_stubs_locally(_name@0_0).

sheep_via(_name@0_0) ->
    via({'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'()},
        _name@0_0).

dog_via(_name@0_0) ->
    via({'Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'()},
        _name@0_0).

sheep_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::name'('Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
                                                            'Defaults::Animal::returns_self'('Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'(),
                                                                                             _name@0_0)).

dog_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::name'('Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
                                                          'Defaults::Animal::returns_self'('Elixir.Patina.Defaults.Dog':'Defaults::Animal::$patina_witness'(),
                                                                                           _name@0_0)).

'Defaults::Combine::uses_rhs_witness'(_witness@0_0,
                                      _witness@1_0, _self@2_0, _other@3_0) ->
    begin
        _witness@4_0 = erlang:element(1,
                                      erlang:element(2, _witness@0_0)),
        (erlang:element(1,
                        erlang:element(1, _witness@4_0)))(_witness@4_0,
                                                          _self@2_0)
    end
        +
        begin
            _witness@5_0 = erlang:element(1, _witness@1_0),
            (erlang:element(1,
                            erlang:element(1, _witness@5_0)))(_witness@5_0,
                                                              _other@3_0)
        end.

combine_via(_witness@0_0, _a@1_0, _b@2_0) ->
    begin
        _witness@3_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                          {erlang:element(2,
                                                                          _witness@0_0)},
                                                          _a@1_0,
                                                          _b@2_0)
    end.

combine_coin_piles(_a@0_0, _b@1_0) ->
    'Defaults::Combine::uses_rhs_witness'('Elixir.Patina.Defaults.Pile':'Defaults::Combine::$patina_witness'({'Elixir.Patina.Defaults.Coin':'Defaults::Value::$patina_witness'()}),
                                          {'Elixir.Patina.Defaults.Pile':'Defaults::Value::$patina_witness'({'Elixir.Patina.Defaults.Coin':'Defaults::Value::$patina_witness'()})},
                                          begin _0@2_0 = _a@0_0, _0@2_0 end,
                                          begin _0@3_0 = _b@1_0, _0@3_0 end).

combine_coin_with_note(_a@0_0, _b@1_0) ->
    'Defaults::Combine::uses_rhs_witness'('Elixir.Patina.Defaults.Coin':'Defaults::Combine<Defaults::Note>::$patina_witness'(),
                                          {'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'()},
                                          _a@0_0,
                                          _b@1_0).

combine_note_piles_via(_a@0_0, _b@1_0) ->
    combine_via({'Elixir.Patina.Defaults.Pile':'Defaults::Combine::$patina_witness'({'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'()}),
                 'Elixir.Patina.Defaults.Pile':'Defaults::Value::$patina_witness'({'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'()})},
                begin _0@2_0 = _a@0_0, _0@2_0 end,
                begin _0@3_0 = _b@1_0, _0@3_0 end).

combine_coin_with_note_via(_a@0_0, _b@1_0) ->
    combine_via({'Elixir.Patina.Defaults.Coin':'Defaults::Combine<Defaults::Note>::$patina_witness'(),
                 'Elixir.Patina.Defaults.Note':'Defaults::Value::$patina_witness'()},
                _a@0_0,
                _b@1_0).

double(_x@0_0) ->
    'Elixir.Patina.Defaults.Inner':'Defaults::Inner::Doubler::doubled'('Elixir.Patina.Defaults.N':'Defaults::Inner::Doubler::$patina_witness'(),
                                                                       _x@0_0).

'Defaults::Greeter::greet'(_witness@0_0, _self@1_0) ->
    begin
        _element@3_0 = begin
                           _witness@2_0 = erlang:element(1,
                                                         erlang:element(2,
                                                                        _witness@0_0)),
                           (erlang:element(1,
                                           erlang:element(1,
                                                          _witness@2_0)))(_witness@2_0,
                                                                          _self@1_0)
                       end,
        {<<"Hello"/utf8>>, _element@3_0}
    end.

greet_via(_witness@0_0, _greeter@1_0) ->
    begin
        _witness@2_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                          _greeter@1_0)
    end.

greet(_name@0_0) ->
    'Defaults::Greeter::greet'('Elixir.Patina.Defaults.Person':'Defaults::Greeter::$patina_witness'(),
                               _name@0_0).

greet_through_witness(_name@0_0) ->
    greet_via({'Elixir.Patina.Defaults.Person':'Defaults::Greeter::$patina_witness'()},
              _name@0_0).

'Defaults::Container::needs_item_witness'(_witness@0_0,
                                          _witness@1_0, _self@2_0) ->
    begin
        _witness@3_0 = erlang:element(1, _witness@1_0),
        (erlang:element(1,
                        erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                          (erlang:element(2,
                                                                          erlang:element(1,
                                                                                         _witness@0_0)))(_witness@0_0,
                                                                                                         _self@2_0))
    end.

first(_witness@0_0, _container@1_0) ->
    begin
        _witness@3_0 = erlang:element(2, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                          begin
                                                              _witness@2_0 =
                                                                  erlang:element(1,
                                                                                 _witness@0_0),
                                                              (erlang:element(2,
                                                                              erlang:element(1,
                                                                                             _witness@2_0)))(_witness@2_0,
                                                                                                             _container@1_0)
                                                          end)
    end.

nested(_witness@0_0, _container@1_0) ->
    begin
        _witness@4_0 = erlang:element(3, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@4_0)))(_witness@4_0,
                                                          begin
                                                              _witness@3_0 =
                                                                  erlang:element(2,
                                                                                 _witness@0_0),
                                                              (erlang:element(2,
                                                                              erlang:element(1,
                                                                                             _witness@3_0)))(_witness@3_0,
                                                                                                             begin
                                                                                                                 _witness@2_0 =
                                                                                                                     erlang:element(1,
                                                                                                                                    _witness@0_0),
                                                                                                                 (erlang:element(2,
                                                                                                                                 erlang:element(1,
                                                                                                                                                _witness@2_0)))(_witness@2_0,
                                                                                                                                                                _container@1_0)
                                                                                                             end)
                                                          end)
    end.

describe_boxed(_name@0_0) ->
    'Defaults::Container::needs_item_witness'('Elixir.Patina.Defaults.Boxed':'Defaults::Container::$patina_witness'(),
                                              {'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
                                              begin
                                                  _0@1_0 = _name@0_0, _0@1_0
                                              end).

describe_wrapped(_name@0_0) ->
    'Defaults::Container::needs_item_witness'('Elixir.Patina.Defaults.Wrapped':'Defaults::Container::$patina_witness'({'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()}),
                                              {'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
                                              begin
                                                  _0@1_0 = _name@0_0, _0@1_0
                                              end).

describe_rewrapped(_name@0_0) ->
    'Defaults::Container::needs_item_witness'('Elixir.Patina.Defaults.Rewrapped':'Defaults::Container::$patina_witness'(),
                                              {'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
                                              begin
                                                  _0@2_0 = begin
                                                               _0@1_0 =
                                                                   _name@0_0,
                                                               _0@1_0
                                                           end,
                                                  _0@2_0
                                              end).

describe_loose(_name@0_0) ->
    'Defaults::Container::needs_item_witness'('Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
                                              {'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
                                              begin
                                                  _0@1_0 = _name@0_0, _0@1_0
                                              end).

describe_first_loose(_name@0_0) ->
    first({'Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
           'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
          begin _0@1_0 = _name@0_0, _0@1_0 end).

describe_nested(_name@0_0) ->
    nested({'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::$patina_witness'({'Elixir.Patina.Defaults.Boxed':'Defaults::Describe::$patina_witness'()}),
            'Elixir.Patina.Defaults.Boxed':'Defaults::Container::$patina_witness'(),
            'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
           begin
               _0@2_0 = begin _0@1_0 = _name@0_0, _0@1_0 end, _0@2_0
           end).

'Defaults::Maker::make'(_witness@0_0, _self@1_0) ->
    <<"made"/utf8>>.

make() ->
    _ =
        'Defaults::Maker::make'('Elixir.Patina.Defaults.Boxed':'Defaults::Maker::$patina_witness'(),
                                begin _0@0_0 = <<"unused"/utf8>>, _0@0_0 end),
    {}.

'Defaults::Numbered::$patina_const::D'(_witness@0_0) ->
    100.

'Defaults::Numbered::consts'(_witness@0_0, _self@1_0) ->
    begin
        _element@2_0 = (erlang:element(1,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0),
        _element@3_0 = (erlang:element(2,
                                       erlang:element(1,
                                                      _witness@0_0)))(_witness@0_0),
        {_element@2_0, _element@3_0}
    end.

sum(_witness@0_0, __t@1_0) ->
    begin
        _witness@2_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0)
    end
        +
        begin
            _witness@3_0 = erlang:element(1, _witness@0_0),
            (erlang:element(2,
                            erlang:element(1, _witness@3_0)))(_witness@3_0)
        end.

one_consts() ->
    'Defaults::Numbered::consts'('Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'(),
                                 'Elixir.Patina.Defaults.One').

two_consts() ->
    'Defaults::Numbered::consts'('Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'(),
                                 'Elixir.Patina.Defaults.Two').

plus_one_consts() ->
    'Defaults::Numbered::consts'('Elixir.Patina.Defaults.Plus':'Defaults::Numbered::$patina_witness'({'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'()}),
                                 begin
                                     _0@0_0 = 'Elixir.Patina.Defaults.One',
                                     _0@0_0
                                 end).

sum_two() ->
    sum({'Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'()},
        'Elixir.Patina.Defaults.Two').

one_c() ->
    'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_const::C'('Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_witness'()).

plus_two_d() ->
    'Defaults::Numbered::$patina_const::D'('Elixir.Patina.Defaults.Plus':'Defaults::Numbered::$patina_witness'({'Elixir.Patina.Defaults.Two':'Defaults::Numbered::$patina_witness'()})).

doubled_k() ->
    'Elixir.Patina.Defaults.Holder':'Defaults::Konst::$patina_const::K'('Elixir.Patina.Defaults.Holder':'Defaults::Konst::$patina_witness'())
        * 2.

differ(_witness@0_0, _a@1_0, _b@2_0) ->
    begin
        _witness@3_0 = erlang:element(1, _witness@0_0),
        (erlang:element(2,
                        erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                          _a@1_0,
                                                          _b@2_0)
    end.

int_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.Int':'Core::PartialEq::ne'('Elixir.Patina.Core.Int':'Core::PartialEq::$patina_witness'(),
                                                   _a@0_0,
                                                   _b@1_0).

str_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.Str':'Core::PartialEq::ne'('Elixir.Patina.Core.Str':'Core::PartialEq::$patina_witness'(),
                                                   _a@0_0,
                                                   _b@1_0).

p_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core':'Core::PartialEq::ne'('Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'(),
                                               _a@0_0,
                                               _b@1_0).

p_differ(_a@0_0, _b@1_0) ->
    differ({'Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'()},
           _a@0_0,
           _b@1_0).

int_differ(_a@0_0, _b@1_0) ->
    differ({'Elixir.Patina.Core.Int':'Core::PartialEq::$patina_witness'()},
           _a@0_0,
           _b@1_0).

ring_via(_witness@0_0, _ringer@1_0) ->
    begin
        _witness@2_0 = erlang:element(1, _witness@0_0),
        (erlang:element(1,
                        erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                          {'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'()},
                                                          _ringer@1_0,
                                                          'Elixir.Patina.Defaults.Bell')
    end.

ring_loud() ->
    ring_via({'Elixir.Patina.Defaults.Loud':'Defaults::Ring::$patina_witness'()},
             'Elixir.Patina.Defaults.Loud').

ring_quiet() ->
    ring_via({'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::$patina_witness'()},
             'Elixir.Patina.Defaults.Quiet').

ring_quiet_directly() ->
    'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::ring'('Elixir.Patina.Defaults.Quiet':'Defaults::Ring::$patina_witness'(),
                                                          {'Elixir.Patina.Defaults.Bell':'Defaults::Tag::$patina_witness'()},
                                                          'Elixir.Patina.Defaults.Quiet',
                                                          'Elixir.Patina.Defaults.Bell').

calls_required_through_pointer(_witness@0_0,
                               _animal@1_0) ->
    _calls_required@4_0 = fun (_arg@2_0) ->
                                  _witness@3_0 = erlang:element(1,
                                                                _witness@0_0),
                                  (erlang:element(3,
                                                  erlang:element(1,
                                                                 _witness@3_0)))(_witness@3_0,
                                                                                 _arg@2_0)
                          end,
    _calls_required@4_0(_animal@1_0).

describe_through_pointer(_name@0_0) ->
    _describe@2_0 = fun (_arg@1_0) ->
                            first({'Elixir.Patina.Defaults.Loose':'Defaults::Container::$patina_witness'(),
                                   'Elixir.Patina.Defaults.Name':'Defaults::Describe::$patina_witness'()},
                                  _arg@1_0)
                    end,
    _describe@2_0(begin _0@3_0 = _name@0_0, _0@3_0 end).

ne_through_pointer(_a@0_0, _b@1_0) ->
    _ne@4_0 = fun (_arg@2_0, _arg@3_0) ->
                      'Elixir.Patina.Core':'Core::PartialEq::ne'('Elixir.Patina.Defaults.P':'Core::PartialEq::$patina_witness'(),
                                                                 _arg@2_0,
                                                                 _arg@3_0)
              end,
    _ne@4_0(_a@0_0, _b@1_0).

sheep_calls_required_through_pointer(_name@0_0) ->
    calls_required_through_pointer({'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::$patina_witness'()},
                                   _name@0_0).
