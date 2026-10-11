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
    {(erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::name'(_witness@0_0,
                                                             _self@1_0),
     (erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::noise'(_witness@0_0,
                                                              _self@1_0)}.

'Defaults::Animal::calls_default'(_witness@0_0,
                                  _self@1_0) ->
    {(erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::calls_required'(_witness@0_0,
                                                                       _self@1_0),
     (erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::calls_required'(_witness@0_0,
                                                                       _self@1_0)}.

'Defaults::Animal::with_bounded_param'(_witness@0_0,
                                       _witness@2_0 = {_witness@1_0}, _self@3_0,
                                       _tag@4_0) ->
    {(erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::name'(_witness@0_0,
                                                             _self@3_0),
     (erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::noise'(_witness@0_0,
                                                              _self@3_0),
     (erlang:element(2,
                     _witness@1_0)):'Defaults::Tag::tag'(_witness@1_0,
                                                         _tag@4_0)}.

'Defaults::Animal::returns_self'(_witness@0_0,
                                 _self@1_0) ->
    _this@2_0 = _self@1_0,
    _this@2_0.

'Defaults::Animal::whisper'(_witness@0_0, _self@1_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Defaults::Animal::name'(_witness@0_0,
                                                            _self@1_0).

via(_witness@1_0 = {_witness@0_0}, _animal@2_0) ->
    {(erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::calls_required'(_witness@0_0,
                                                                       _animal@2_0),
     (erlang:element(2,
                     _witness@0_0)):'Defaults::Animal::calls_default'(_witness@0_0,
                                                                      _animal@2_0)}.

sheep_calls_required(_name@0_0) ->
    'Defaults::Animal::calls_required'({'impl Defaults::Animal for Defaults::Sheep',
                                        'Elixir.Patina.Defaults.Sheep'},
                                       _name@0_0).

dog_calls_required(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::calls_required'({'impl Defaults::Animal for Defaults::Dog',
                                                                     'Elixir.Patina.Defaults.Dog'},
                                                                    _name@0_0).

sheep_with_bounded_param(_name@0_0) ->
    'Defaults::Animal::with_bounded_param'({'impl Defaults::Animal for Defaults::Sheep',
                                            'Elixir.Patina.Defaults.Sheep'},
                                           {{'impl Defaults::Tag for Defaults::Bell',
                                             'Elixir.Patina.Defaults.Bell'}},
                                           _name@0_0,
                                           'Elixir.Patina.Defaults.Bell').

dog_with_bounded_param(_name@0_0) ->
    'Defaults::Animal::with_bounded_param'({'impl Defaults::Animal for Defaults::Dog',
                                            'Elixir.Patina.Defaults.Dog'},
                                           {{'impl Defaults::Tag for Defaults::Bell',
                                             'Elixir.Patina.Defaults.Bell'}},
                                           _name@0_0,
                                           'Elixir.Patina.Defaults.Bell').

sheep_calls_stubs_locally(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':calls_stubs_locally(_name@0_0).

sheep_via(_name@0_0) ->
    via({{'impl Defaults::Animal for Defaults::Sheep',
          'Elixir.Patina.Defaults.Sheep'}},
        _name@0_0).

dog_via(_name@0_0) ->
    via({{'impl Defaults::Animal for Defaults::Dog',
          'Elixir.Patina.Defaults.Dog'}},
        _name@0_0).

sheep_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::name'({'impl Defaults::Animal for Defaults::Sheep',
                                                             'Elixir.Patina.Defaults.Sheep'},
                                                            'Defaults::Animal::returns_self'({'impl Defaults::Animal for Defaults::Sheep',
                                                                                              'Elixir.Patina.Defaults.Sheep'},
                                                                                             _name@0_0)).

dog_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::name'({'impl Defaults::Animal for Defaults::Dog',
                                                           'Elixir.Patina.Defaults.Dog'},
                                                          'Defaults::Animal::returns_self'({'impl Defaults::Animal for Defaults::Dog',
                                                                                            'Elixir.Patina.Defaults.Dog'},
                                                                                           _name@0_0)).

'Defaults::Combine::uses_rhs_witness'(_witness@0_0,
                                      _witness@2_0 = {_witness@1_0}, _self@3_0,
                                      _other@4_0) ->
    begin
        _witness@5_0 = erlang:element(3, _witness@0_0),
        (erlang:element(2,
                        _witness@5_0)):'Defaults::Value::value'(_witness@5_0,
                                                                _self@3_0)
    end
        +
        (erlang:element(2,
                        _witness@1_0)):'Defaults::Value::value'(_witness@1_0,
                                                                _other@4_0).

combine_via(_witness@2_0 = {_witness@0_0, _witness@1_0},
            _a@3_0, _b@4_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Defaults::Combine::uses_rhs_witness'(_witness@0_0,
                                                                         {_witness@1_0},
                                                                         _a@3_0,
                                                                         _b@4_0).

combine_coin_piles(_a@0_0, _b@1_0) ->
    'Defaults::Combine::uses_rhs_witness'({'impl Defaults::Combine for Defaults::Pile<_>',
                                           'Elixir.Patina.Defaults.Pile',
                                           {'impl Defaults::Value for Defaults::Pile<_>',
                                            'Elixir.Patina.Defaults.Pile',
                                            {'impl Defaults::Value for Defaults::Coin',
                                             'Elixir.Patina.Defaults.Coin'}},
                                           {'impl Defaults::Value for Defaults::Coin',
                                            'Elixir.Patina.Defaults.Coin'}},
                                          {{'impl Defaults::Value for Defaults::Pile<_>',
                                            'Elixir.Patina.Defaults.Pile',
                                            {'impl Defaults::Value for Defaults::Coin',
                                             'Elixir.Patina.Defaults.Coin'}}},
                                          _a@0_0,
                                          _b@1_0).

combine_coin_with_note(_a@0_0, _b@1_0) ->
    'Defaults::Combine::uses_rhs_witness'({'impl Defaults::Combine<Defaults::Note> for Defaults::Coin',
                                           'Elixir.Patina.Defaults.Coin',
                                           {'impl Defaults::Value for Defaults::Coin',
                                            'Elixir.Patina.Defaults.Coin'}},
                                          {{'impl Defaults::Value for Defaults::Note',
                                            'Elixir.Patina.Defaults.Note'}},
                                          _a@0_0,
                                          _b@1_0).

combine_note_piles_via(_a@0_0, _b@1_0) ->
    combine_via({{'impl Defaults::Combine for Defaults::Pile<_>',
                  'Elixir.Patina.Defaults.Pile',
                  {'impl Defaults::Value for Defaults::Pile<_>',
                   'Elixir.Patina.Defaults.Pile',
                   {'impl Defaults::Value for Defaults::Note',
                    'Elixir.Patina.Defaults.Note'}},
                  {'impl Defaults::Value for Defaults::Note',
                   'Elixir.Patina.Defaults.Note'}},
                 {'impl Defaults::Value for Defaults::Pile<_>',
                  'Elixir.Patina.Defaults.Pile',
                  {'impl Defaults::Value for Defaults::Note',
                   'Elixir.Patina.Defaults.Note'}}},
                _a@0_0,
                _b@1_0).

combine_coin_with_note_via(_a@0_0, _b@1_0) ->
    combine_via({{'impl Defaults::Combine<Defaults::Note> for Defaults::Coin',
                  'Elixir.Patina.Defaults.Coin',
                  {'impl Defaults::Value for Defaults::Coin',
                   'Elixir.Patina.Defaults.Coin'}},
                 {'impl Defaults::Value for Defaults::Note',
                  'Elixir.Patina.Defaults.Note'}},
                _a@0_0,
                _b@1_0).

double(_x@0_0) ->
    'Elixir.Patina.Defaults.Inner':'Defaults::Inner::Doubler::doubled'({'impl Defaults::Inner::Doubler for Defaults::N',
                                                                        'Elixir.Patina.Defaults.N'},
                                                                       _x@0_0).

'Defaults::Greeter::greet'(_witness@0_0, _self@1_0) ->
    {<<"Hello"/utf8>>,
     begin
         _witness@2_0 = erlang:element(3, _witness@0_0),
         (erlang:element(2,
                         _witness@2_0)):'Defaults::Named::name'(_witness@2_0,
                                                                _self@1_0)
     end}.

greet_via(_witness@1_0 = {_witness@0_0},
          _greeter@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Defaults::Greeter::greet'(_witness@0_0,
                                                              _greeter@2_0).

greet(_name@0_0) ->
    'Defaults::Greeter::greet'({'impl Defaults::Greeter for Defaults::Person',
                                'Elixir.Patina.Defaults.Person',
                                {'impl Defaults::Named for Defaults::Person',
                                 'Elixir.Patina.Defaults.Person'}},
                               _name@0_0).

greet_through_witness(_name@0_0) ->
    greet_via({{'impl Defaults::Greeter for Defaults::Person',
                'Elixir.Patina.Defaults.Person',
                {'impl Defaults::Named for Defaults::Person',
                 'Elixir.Patina.Defaults.Person'}}},
              _name@0_0).

'Defaults::Container::needs_item_witness'(_witness@0_0,
                                          _witness@2_0 = {_witness@1_0},
                                          _self@3_0) ->
    (erlang:element(2,
                    _witness@1_0)):'Defaults::Describe::describe'(_witness@1_0,
                                                                  (erlang:element(2,
                                                                                  _witness@0_0)):'Defaults::Container::get'(_witness@0_0,
                                                                                                                            _self@3_0)).

first(_witness@2_0 = {_witness@0_0, _witness@1_0},
      _container@3_0) ->
    (erlang:element(2,
                    _witness@1_0)):'Defaults::Describe::describe'(_witness@1_0,
                                                                  (erlang:element(2,
                                                                                  _witness@0_0)):'Defaults::Container::get'(_witness@0_0,
                                                                                                                            _container@3_0)).

nested(_witness@3_0 = {_witness@0_0,
                       _witness@1_0,
                       _witness@2_0},
       _container@4_0) ->
    (erlang:element(2,
                    _witness@2_0)):'Defaults::Describe::describe'(_witness@2_0,
                                                                  (erlang:element(2,
                                                                                  _witness@1_0)):'Defaults::Container::get'(_witness@1_0,
                                                                                                                            (erlang:element(2,
                                                                                                                                            _witness@0_0)):'Defaults::Container::get'(_witness@0_0,
                                                                                                                                                                                      _container@4_0))).

describe_boxed(_name@0_0) ->
    'Defaults::Container::needs_item_witness'({'impl Defaults::Container for Defaults::Boxed',
                                               'Elixir.Patina.Defaults.Boxed'},
                                              {{'impl Defaults::Describe for Defaults::Name',
                                                'Elixir.Patina.Defaults.Name'}},
                                              _name@0_0).

describe_wrapped(_name@0_0) ->
    'Defaults::Container::needs_item_witness'({'impl Defaults::Container for Defaults::Wrapped<_>',
                                               'Elixir.Patina.Defaults.Wrapped',
                                               {'impl Defaults::Describe for Defaults::Name',
                                                'Elixir.Patina.Defaults.Name'}},
                                              {{'impl Defaults::Describe for Defaults::Name',
                                                'Elixir.Patina.Defaults.Name'}},
                                              _name@0_0).

describe_rewrapped(_name@0_0) ->
    'Defaults::Container::needs_item_witness'({'impl Defaults::Container for Defaults::Rewrapped',
                                               'Elixir.Patina.Defaults.Rewrapped'},
                                              {{'impl Defaults::Describe for Defaults::Name',
                                                'Elixir.Patina.Defaults.Name'}},
                                              _name@0_0).

describe_loose(_name@0_0) ->
    'Defaults::Container::needs_item_witness'({'impl Defaults::Container for Defaults::Loose<_>',
                                               'Elixir.Patina.Defaults.Loose'},
                                              {{'impl Defaults::Describe for Defaults::Name',
                                                'Elixir.Patina.Defaults.Name'}},
                                              _name@0_0).

describe_first_loose(_name@0_0) ->
    first({{'impl Defaults::Container for Defaults::Loose<_>',
            'Elixir.Patina.Defaults.Loose'},
           {'impl Defaults::Describe for Defaults::Name',
            'Elixir.Patina.Defaults.Name'}},
          _name@0_0).

describe_nested(_name@0_0) ->
    nested({{'impl Defaults::Container for Defaults::Wrapped<_>',
             'Elixir.Patina.Defaults.Wrapped',
             {'impl Defaults::Describe for Defaults::Boxed',
              'Elixir.Patina.Defaults.Boxed'}},
            {'impl Defaults::Container for Defaults::Boxed',
             'Elixir.Patina.Defaults.Boxed'},
            {'impl Defaults::Describe for Defaults::Name',
             'Elixir.Patina.Defaults.Name'}},
           _name@0_0).

'Defaults::Maker::make'(_witness@0_0, _self@1_0) ->
    <<"made"/utf8>>.

make() ->
    _ =
        'Defaults::Maker::make'({'impl Defaults::Maker for Defaults::Boxed',
                                 'Elixir.Patina.Defaults.Boxed'},
                                <<"unused"/utf8>>),
    {}.

'Defaults::Numbered::$patina_const::D'(_witness@0_0) ->
    100.

'Defaults::Numbered::consts'(_witness@0_0, _self@1_0) ->
    {(erlang:element(2,
                     _witness@0_0)):'Defaults::Numbered::$patina_const::C'(_witness@0_0),
     (erlang:element(2,
                     _witness@0_0)):'Defaults::Numbered::$patina_const::D'(_witness@0_0)}.

sum(_witness@1_0 = {_witness@0_0}, __t@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Defaults::Numbered::$patina_const::C'(_witness@0_0)
        +
        (erlang:element(2,
                        _witness@0_0)):'Defaults::Numbered::$patina_const::D'(_witness@0_0).

one_consts() ->
    'Defaults::Numbered::consts'({'impl Defaults::Numbered for Defaults::One',
                                  'Elixir.Patina.Defaults.One'},
                                 'Elixir.Patina.Defaults.One').

two_consts() ->
    'Defaults::Numbered::consts'({'impl Defaults::Numbered for Defaults::Two',
                                  'Elixir.Patina.Defaults.Two'},
                                 'Elixir.Patina.Defaults.Two').

plus_one_consts() ->
    'Defaults::Numbered::consts'({'impl Defaults::Numbered for Defaults::Plus<_>',
                                  'Elixir.Patina.Defaults.Plus',
                                  {'impl Defaults::Numbered for Defaults::One',
                                   'Elixir.Patina.Defaults.One'}},
                                 'Elixir.Patina.Defaults.One').

sum_two() ->
    sum({{'impl Defaults::Numbered for Defaults::Two',
          'Elixir.Patina.Defaults.Two'}},
        'Elixir.Patina.Defaults.Two').

one_c() ->
    'Elixir.Patina.Defaults.One':'Defaults::Numbered::$patina_const::C'({'impl Defaults::Numbered for Defaults::One',
                                                                         'Elixir.Patina.Defaults.One'}).

plus_two_d() ->
    'Defaults::Numbered::$patina_const::D'({'impl Defaults::Numbered for Defaults::Plus<_>',
                                            'Elixir.Patina.Defaults.Plus',
                                            {'impl Defaults::Numbered for Defaults::Two',
                                             'Elixir.Patina.Defaults.Two'}}).

doubled_k() ->
    'Elixir.Patina.Defaults.Holder':'Defaults::Konst::$patina_const::K'({'impl Defaults::Konst for Defaults::Holder',
                                                                         'Elixir.Patina.Defaults.Holder'})
        * 2.

differ(_witness@1_0 = {_witness@0_0}, _a@2_0, _b@3_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Core::PartialEq::ne'(_witness@0_0,
                                                         _a@2_0,
                                                         _b@3_0).

int_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.Int':'Core::PartialEq::ne'({'impl Core::PartialEq for int',
                                                    'Elixir.Patina.Core.Int'},
                                                   _a@0_0,
                                                   _b@1_0).

str_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.Str':'Core::PartialEq::ne'({'impl Core::PartialEq for str',
                                                    'Elixir.Patina.Core.Str'},
                                                   _a@0_0,
                                                   _b@1_0).

p_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core':'Core::PartialEq::ne'({'impl Core::PartialEq for Defaults::P',
                                                'Elixir.Patina.Defaults.P'},
                                               _a@0_0,
                                               _b@1_0).

p_differ(_a@0_0, _b@1_0) ->
    differ({{'impl Core::PartialEq for Defaults::P',
             'Elixir.Patina.Defaults.P'}},
           _a@0_0,
           _b@1_0).

int_differ(_a@0_0, _b@1_0) ->
    differ({{'impl Core::PartialEq for int',
             'Elixir.Patina.Core.Int'}},
           _a@0_0,
           _b@1_0).

ring_via(_witness@1_0 = {_witness@0_0}, _ringer@2_0) ->
    (erlang:element(2,
                    _witness@0_0)):'Defaults::Ring::ring'(_witness@0_0,
                                                          {{'impl Defaults::Tag for Defaults::Bell',
                                                            'Elixir.Patina.Defaults.Bell'}},
                                                          _ringer@2_0,
                                                          'Elixir.Patina.Defaults.Bell').

ring_loud() ->
    ring_via({{'impl Defaults::Ring for Defaults::Loud',
               'Elixir.Patina.Defaults.Loud'}},
             'Elixir.Patina.Defaults.Loud').

ring_quiet() ->
    ring_via({{'impl Defaults::Ring for Defaults::Quiet',
               'Elixir.Patina.Defaults.Quiet'}},
             'Elixir.Patina.Defaults.Quiet').

ring_quiet_directly() ->
    'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::ring'({'impl Defaults::Ring for Defaults::Quiet',
                                                           'Elixir.Patina.Defaults.Quiet'},
                                                          {{'impl Defaults::Tag for Defaults::Bell',
                                                            'Elixir.Patina.Defaults.Bell'}},
                                                          'Elixir.Patina.Defaults.Quiet',
                                                          'Elixir.Patina.Defaults.Bell').

calls_required_through_pointer(_witness@1_0 =
                                   {_witness@0_0},
                               _animal@2_0) ->
    _calls_required@4_0 = fun (_arg@3_0) ->
                                  (erlang:element(2,
                                                  _witness@0_0)):'Defaults::Animal::calls_required'(_witness@0_0,
                                                                                                    _arg@3_0)
                          end,
    _calls_required@4_0(_animal@2_0).

describe_through_pointer(_name@0_0) ->
    _describe@2_0 = fun (_arg@1_0) ->
                            first({{'impl Defaults::Container for Defaults::Loose<_>',
                                    'Elixir.Patina.Defaults.Loose'},
                                   {'impl Defaults::Describe for Defaults::Name',
                                    'Elixir.Patina.Defaults.Name'}},
                                  _arg@1_0)
                    end,
    _describe@2_0(_name@0_0).

ne_through_pointer(_a@0_0, _b@1_0) ->
    _ne@4_0 = fun (_arg@2_0, _arg@3_0) ->
                      'Elixir.Patina.Core':'Core::PartialEq::ne'({'impl Core::PartialEq for Defaults::P',
                                                                  'Elixir.Patina.Defaults.P'},
                                                                 _arg@2_0,
                                                                 _arg@3_0)
              end,
    _ne@4_0(_a@0_0, _b@1_0).

sheep_calls_required_through_pointer(_name@0_0) ->
    calls_required_through_pointer({{'impl Defaults::Animal for Defaults::Sheep',
                                     'Elixir.Patina.Defaults.Sheep'}},
                                   _name@0_0).
