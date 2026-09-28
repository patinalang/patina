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
         'Defaults::Container::needs_item_witness'/2,
         describe_boxed/1,
         describe_wrapped/1,
         describe_rewrapped/1,
         describe_loose/1,
         describe_first_loose/1,
         describe_nested/1,
         'Defaults::Maker::make'/2,
         make/0,
         'Defaults::Numbered::const::D'/1,
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

'Defaults::Animal::calls_required'(_Self@0_0,
                                   _self@1_0) ->
    {(erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::name'(_Self@0_0,
                                                          _self@1_0),
     (erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::noise'(_Self@0_0,
                                                           _self@1_0)}.

'Defaults::Animal::calls_default'(_Self@0_0,
                                  _self@1_0) ->
    {(erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::calls_required'(_Self@0_0,
                                                                    _self@1_0),
     (erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::calls_required'(_Self@0_0,
                                                                    _self@1_0)}.

'Defaults::Animal::with_bounded_param'(_Self@0_0,
                                       _T@1_0, _self@2_0, _tag@3_0) ->
    {(erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::name'(_Self@0_0,
                                                          _self@2_0),
     (erlang:element(1,
                     _Self@0_0)):'Defaults::Animal::noise'(_Self@0_0,
                                                           _self@2_0),
     (erlang:element(1, _T@1_0)):'Defaults::Tag::tag'(_T@1_0,
                                                      _tag@3_0)}.

'Defaults::Animal::returns_self'(_Self@0_0,
                                 _self@1_0) ->
    _this@2_0 = _self@1_0,
    _this@2_0.

'Defaults::Animal::whisper'(_Self@0_0, _self@1_0) ->
    (erlang:element(1,
                    _Self@0_0)):'Defaults::Animal::name'(_Self@0_0,
                                                         _self@1_0).

via(_T@0_0, _animal@1_0) ->
    {(erlang:element(1,
                     _T@0_0)):'Defaults::Animal::calls_required'(_T@0_0,
                                                                 _animal@1_0),
     (erlang:element(1,
                     _T@0_0)):'Defaults::Animal::calls_default'(_T@0_0,
                                                                _animal@1_0)}.

sheep_calls_required(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::calls_required'({'Elixir.Patina.Defaults.Sheep',
                                                                       {}},
                                                                      _name@0_0).

dog_calls_required(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::calls_required'({'Elixir.Patina.Defaults.Dog',
                                                                     {}},
                                                                    _name@0_0).

sheep_with_bounded_param(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::with_bounded_param'({'Elixir.Patina.Defaults.Sheep',
                                                                           {}},
                                                                          {'Elixir.Patina.Defaults.Bell',
                                                                           {}},
                                                                          _name@0_0,
                                                                          'Elixir.Patina.Defaults.Bell').

dog_with_bounded_param(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::with_bounded_param'({'Elixir.Patina.Defaults.Dog',
                                                                         {}},
                                                                        {'Elixir.Patina.Defaults.Bell',
                                                                         {}},
                                                                        _name@0_0,
                                                                        'Elixir.Patina.Defaults.Bell').

sheep_calls_stubs_locally(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':calls_stubs_locally(_name@0_0).

sheep_via(_name@0_0) ->
    via({'Elixir.Patina.Defaults.Sheep', {}}, _name@0_0).

dog_via(_name@0_0) ->
    via({'Elixir.Patina.Defaults.Dog', {}}, _name@0_0).

sheep_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::name'({'Elixir.Patina.Defaults.Sheep',
                                                             {}},
                                                            'Elixir.Patina.Defaults.Sheep':'Defaults::Animal::returns_self'({'Elixir.Patina.Defaults.Sheep',
                                                                                                                             {}},
                                                                                                                            _name@0_0)).

dog_returns_self(_name@0_0) ->
    'Elixir.Patina.Defaults.Dog':'Defaults::Animal::name'({'Elixir.Patina.Defaults.Dog',
                                                           {}},
                                                          'Elixir.Patina.Defaults.Dog':'Defaults::Animal::returns_self'({'Elixir.Patina.Defaults.Dog',
                                                                                                                         {}},
                                                                                                                        _name@0_0)).

'Defaults::Combine::uses_rhs_witness'(_Self@0_0,
                                      _Rhs@1_0, _self@2_0, _other@3_0) ->
    (erlang:element(1,
                    _Self@0_0)):'Defaults::Value::value'(_Self@0_0,
                                                         _self@2_0)
        +
        (erlang:element(1,
                        _Rhs@1_0)):'Defaults::Value::value'(_Rhs@1_0,
                                                            _other@3_0).

combine_via(_A@0_0, _B@1_0, _a@2_0, _b@3_0) ->
    (erlang:element(1,
                    _A@0_0)):'Defaults::Combine::uses_rhs_witness'(_A@0_0,
                                                                   _B@1_0,
                                                                   _a@2_0,
                                                                   _b@3_0).

combine_coin_piles(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Defaults.Pile':'Defaults::Combine::uses_rhs_witness'({'Elixir.Patina.Defaults.Pile',
                                                                         {{'Elixir.Patina.Defaults.Coin',
                                                                           {}}}},
                                                                        {'Elixir.Patina.Defaults.Pile',
                                                                         {{'Elixir.Patina.Defaults.Coin',
                                                                           {}}}},
                                                                        _a@0_0,
                                                                        _b@1_0).

combine_coin_with_note(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Defaults.Coin':'Defaults::Combine::uses_rhs_witness'({'Elixir.Patina.Defaults.Coin',
                                                                         {}},
                                                                        {'Elixir.Patina.Defaults.Note',
                                                                         {}},
                                                                        _a@0_0,
                                                                        _b@1_0).

combine_note_piles_via(_a@0_0, _b@1_0) ->
    combine_via({'Elixir.Patina.Defaults.Pile',
                 {{'Elixir.Patina.Defaults.Note', {}}}},
                {'Elixir.Patina.Defaults.Pile',
                 {{'Elixir.Patina.Defaults.Note', {}}}},
                _a@0_0,
                _b@1_0).

combine_coin_with_note_via(_a@0_0, _b@1_0) ->
    combine_via({'Elixir.Patina.Defaults.Coin', {}},
                {'Elixir.Patina.Defaults.Note', {}},
                _a@0_0,
                _b@1_0).

double(_x@0_0) ->
    'Elixir.Patina.Defaults.N':'Defaults::Inner::Doubler::doubled'({'Elixir.Patina.Defaults.N',
                                                                    {}},
                                                                   _x@0_0).

'Defaults::Greeter::greet'(_Self@0_0, _self@1_0) ->
    {<<"Hello"/utf8>>,
     (erlang:element(1,
                     _Self@0_0)):'Defaults::Named::name'(_Self@0_0,
                                                         _self@1_0)}.

greet_via(_G@0_0, _greeter@1_0) ->
    (erlang:element(1,
                    _G@0_0)):'Defaults::Greeter::greet'(_G@0_0,
                                                        _greeter@1_0).

greet(_name@0_0) ->
    'Elixir.Patina.Defaults.Person':'Defaults::Greeter::greet'({'Elixir.Patina.Defaults.Person',
                                                                {}},
                                                               _name@0_0).

greet_through_witness(_name@0_0) ->
    greet_via({'Elixir.Patina.Defaults.Person', {}},
              _name@0_0).

'Defaults::Container::needs_item_witness'(_Self@0_0,
                                          _self@1_0) ->
    begin
        _witness@2_0 = (erlang:element(1,
                                       _Self@0_0)):'Defaults::Container::type::Item'(_Self@0_0),
        (erlang:element(1,
                        _witness@2_0)):'Defaults::Describe::describe'(_witness@2_0,
                                                                      (erlang:element(1,
                                                                                      _Self@0_0)):'Defaults::Container::get'(_Self@0_0,
                                                                                                                             _self@1_0))
    end.

first(_C@0_0, _container@1_0) ->
    begin
        _witness@2_0 = (erlang:element(1,
                                       _C@0_0)):'Defaults::Container::type::Item'(_C@0_0),
        (erlang:element(1,
                        _witness@2_0)):'Defaults::Describe::describe'(_witness@2_0,
                                                                      (erlang:element(1,
                                                                                      _C@0_0)):'Defaults::Container::get'(_C@0_0,
                                                                                                                          _container@1_0))
    end.

nested(_C@0_0, _container@1_0) ->
    begin
        _witness@4_0 = begin
                           _witness@2_0 = (erlang:element(1,
                                                          _C@0_0)):'Defaults::Container::type::Item'(_C@0_0),
                           (erlang:element(1,
                                           _witness@2_0)):'Defaults::Container::type::Item'(_witness@2_0)
                       end,
        (erlang:element(1,
                        _witness@4_0)):'Defaults::Describe::describe'(_witness@4_0,
                                                                      begin
                                                                          _witness@3_0 =
                                                                              (erlang:element(1,
                                                                                              _C@0_0)):'Defaults::Container::type::Item'(_C@0_0),
                                                                          (erlang:element(1,
                                                                                          _witness@3_0)):'Defaults::Container::get'(_witness@3_0,
                                                                                                                                    (erlang:element(1,
                                                                                                                                                    _C@0_0)):'Defaults::Container::get'(_C@0_0,
                                                                                                                                                                                        _container@1_0))
                                                                      end)
    end.

describe_boxed(_name@0_0) ->
    'Elixir.Patina.Defaults.Boxed':'Defaults::Container::needs_item_witness'({'Elixir.Patina.Defaults.Boxed',
                                                                              {}},
                                                                             _name@0_0).

describe_wrapped(_name@0_0) ->
    'Elixir.Patina.Defaults.Wrapped':'Defaults::Container::needs_item_witness'({'Elixir.Patina.Defaults.Wrapped',
                                                                                {{'Elixir.Patina.Defaults.Name',
                                                                                  {}}}},
                                                                               _name@0_0).

describe_rewrapped(_name@0_0) ->
    'Elixir.Patina.Defaults.Rewrapped':'Defaults::Container::needs_item_witness'({'Elixir.Patina.Defaults.Rewrapped',
                                                                                  {}},
                                                                                 _name@0_0).

describe_loose(_name@0_0) ->
    'Elixir.Patina.Defaults.Loose':'Defaults::Container::needs_item_witness'({'Elixir.Patina.Defaults.Loose',
                                                                              {{'Elixir.Patina.Defaults.Name',
                                                                                {}}}},
                                                                             _name@0_0).

describe_first_loose(_name@0_0) ->
    first({'Elixir.Patina.Defaults.Loose',
           {{'Elixir.Patina.Defaults.Name', {}}}},
          _name@0_0).

describe_nested(_name@0_0) ->
    nested({'Elixir.Patina.Defaults.Wrapped',
            {{'Elixir.Patina.Defaults.Boxed', {}}}},
           _name@0_0).

'Defaults::Maker::make'(_Self@0_0, _self@1_0) ->
    <<"made"/utf8>>.

make() ->
    _ =
        'Elixir.Patina.Defaults.Boxed':'Defaults::Maker::make'({'Elixir.Patina.Defaults.Boxed',
                                                                {}},
                                                               <<"unused"/utf8>>),
    {}.

'Defaults::Numbered::const::D'(_Self@0_0) -> 100.

'Defaults::Numbered::consts'(_Self@0_0, _self@1_0) ->
    {(erlang:element(1,
                     _Self@0_0)):'Defaults::Numbered::const::C'(_Self@0_0),
     (erlang:element(1,
                     _Self@0_0)):'Defaults::Numbered::const::D'(_Self@0_0)}.

sum(_T@0_0, __t@1_0) ->
    (erlang:element(1,
                    _T@0_0)):'Defaults::Numbered::const::C'(_T@0_0)
        +
        (erlang:element(1,
                        _T@0_0)):'Defaults::Numbered::const::D'(_T@0_0).

one_consts() ->
    'Elixir.Patina.Defaults.One':'Defaults::Numbered::consts'({'Elixir.Patina.Defaults.One',
                                                               {}},
                                                              'Elixir.Patina.Defaults.One').

two_consts() ->
    'Elixir.Patina.Defaults.Two':'Defaults::Numbered::consts'({'Elixir.Patina.Defaults.Two',
                                                               {}},
                                                              'Elixir.Patina.Defaults.Two').

plus_one_consts() ->
    'Elixir.Patina.Defaults.Plus':'Defaults::Numbered::consts'({'Elixir.Patina.Defaults.Plus',
                                                                {{'Elixir.Patina.Defaults.One',
                                                                  {}}}},
                                                               'Elixir.Patina.Defaults.One').

sum_two() ->
    sum({'Elixir.Patina.Defaults.Two', {}},
        'Elixir.Patina.Defaults.Two').

one_c() ->
    'Elixir.Patina.Defaults.One':'Defaults::Numbered::const::C'({'Elixir.Patina.Defaults.One',
                                                                 {}}).

plus_two_d() ->
    'Elixir.Patina.Defaults.Plus':'Defaults::Numbered::const::D'({'Elixir.Patina.Defaults.Plus',
                                                                  {{'Elixir.Patina.Defaults.Two',
                                                                    {}}}}).

doubled_k() ->
    'Elixir.Patina.Defaults.Holder':'Defaults::Konst::const::K'({'Elixir.Patina.Defaults.Holder',
                                                                 {}})
        * 2.

differ(_T@0_0, _a@1_0, _b@2_0) ->
    (erlang:element(1,
                    _T@0_0)):'Core::PartialEq::ne'(_T@0_0,
                                                   _T@0_0,
                                                   _a@1_0,
                                                   _b@2_0).

int_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.int':'Core::PartialEq::ne'({'Elixir.Patina.Core.int',
                                                    {}},
                                                   {'Elixir.Patina.Core.int',
                                                    {}},
                                                   _a@0_0,
                                                   _b@1_0).

str_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Core.str':'Core::PartialEq::ne'({'Elixir.Patina.Core.str',
                                                    {}},
                                                   {'Elixir.Patina.Core.str',
                                                    {}},
                                                   _a@0_0,
                                                   _b@1_0).

p_ne(_a@0_0, _b@1_0) ->
    'Elixir.Patina.Defaults.P':'Core::PartialEq::ne'({'Elixir.Patina.Defaults.P',
                                                      {}},
                                                     {'Elixir.Patina.Defaults.P',
                                                      {}},
                                                     _a@0_0,
                                                     _b@1_0).

p_differ(_a@0_0, _b@1_0) ->
    differ({'Elixir.Patina.Defaults.P', {}},
           _a@0_0,
           _b@1_0).

int_differ(_a@0_0, _b@1_0) ->
    differ({'Elixir.Patina.Core.int', {}}, _a@0_0, _b@1_0).

ring_via(_R@0_0, _ringer@1_0) ->
    (erlang:element(1,
                    _R@0_0)):'Defaults::Ring::ring'(_R@0_0,
                                                    {'Elixir.Patina.Defaults.Bell',
                                                     {}},
                                                    _ringer@1_0,
                                                    'Elixir.Patina.Defaults.Bell').

ring_loud() ->
    ring_via({'Elixir.Patina.Defaults.Loud', {}},
             'Elixir.Patina.Defaults.Loud').

ring_quiet() ->
    ring_via({'Elixir.Patina.Defaults.Quiet', {}},
             'Elixir.Patina.Defaults.Quiet').

ring_quiet_directly() ->
    'Elixir.Patina.Defaults.Quiet':'Defaults::Ring::ring'({'Elixir.Patina.Defaults.Quiet',
                                                           {}},
                                                          {'Elixir.Patina.Defaults.Bell',
                                                           {}},
                                                          'Elixir.Patina.Defaults.Quiet',
                                                          'Elixir.Patina.Defaults.Bell').

calls_required_through_pointer(_T@0_0, _animal@1_0) ->
    _calls_required@3_0 = fun (_arg@2_0) ->
                                  (erlang:element(1,
                                                  _T@0_0)):'Defaults::Animal::calls_required'(_T@0_0,
                                                                                              _arg@2_0)
                          end,
    _calls_required@3_0(_animal@1_0).

describe_through_pointer(_name@0_0) ->
    _describe@2_0 = fun (_arg@1_0) ->
                            first({'Elixir.Patina.Defaults.Loose',
                                   {{'Elixir.Patina.Defaults.Name', {}}}},
                                  _arg@1_0)
                    end,
    _describe@2_0(_name@0_0).

ne_through_pointer(_a@0_0, _b@1_0) ->
    _ne@4_0 = fun (_arg@2_0, _arg@3_0) ->
                      'Elixir.Patina.Defaults.P':'Core::PartialEq::ne'({'Elixir.Patina.Defaults.P',
                                                                        {}},
                                                                       {'Elixir.Patina.Defaults.P',
                                                                        {}},
                                                                       _arg@2_0,
                                                                       _arg@3_0)
              end,
    _ne@4_0(_a@0_0, _b@1_0).

sheep_calls_required_through_pointer(_name@0_0) ->
    calls_required_through_pointer({'Elixir.Patina.Defaults.Sheep',
                                    {}},
                                   _name@0_0).
