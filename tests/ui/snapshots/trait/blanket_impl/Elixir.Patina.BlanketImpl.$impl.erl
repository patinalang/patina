-file("tests/ui/src/trait/blanket_impl.pt", 30).

-module('Elixir.Patina.BlanketImpl.$impl').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         'BlanketImpl::Describe::describe'/2,
         'BlanketImpl::Describe::$patina_witness'/1,
         'BlanketImpl::Nested::Twice::twice'/2,
         'BlanketImpl::Nested::Twice::$patina_witness'/1]).

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

'BlanketImpl::Describe::describe'(_witness@0_0,
                                  _self@1_0) ->
    'Elixir.Patina.BlanketImpl':wrap(begin
                                         _witness@2_0 = erlang:element(1,
                                                                       erlang:element(3,
                                                                                      _witness@0_0)),
                                         (erlang:element(1,
                                                         erlang:element(1,
                                                                        _witness@2_0)))(_witness@2_0,
                                                                                        _self@1_0)
                                     end).

'BlanketImpl::Describe::$patina_witness'(_witness@0_0) ->
    {{fun 'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Describe::describe'/2,
      fun 'Elixir.Patina.BlanketImpl':'BlanketImpl::Describe::shout'/2},
     {},
     _witness@0_0}.

'BlanketImpl::Nested::Twice::twice'(_witness@0_0,
                                    _self@1_0) ->
    <<begin
          _witness@2_0 = erlang:element(1,
                                        erlang:element(3, _witness@0_0)),
          (erlang:element(1,
                          erlang:element(1, _witness@2_0)))(_witness@2_0,
                                                            _self@1_0)
      end/binary,
      begin
          _witness@3_0 = erlang:element(1,
                                        erlang:element(3, _witness@0_0)),
          (erlang:element(1,
                          erlang:element(1, _witness@3_0)))(_witness@3_0,
                                                            _self@1_0)
      end/binary>>.

'BlanketImpl::Nested::Twice::$patina_witness'(_witness@0_0) ->
    {{fun 'Elixir.Patina.BlanketImpl.$impl':'BlanketImpl::Nested::Twice::twice'/2},
     {},
     _witness@0_0}.
