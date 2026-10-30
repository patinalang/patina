-file("tests/ui/src/pkg/foreign_structs.pt", 1).

-module('Elixir.Patina.ForeignStructs').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         phantom_tuple/1,
         phantom_tuple_value/1,
         phantom_record/2,
         phantom_record_name/1,
         phantom_record_marker/1,
         rename_phantom_record/1,
         recursive/0,
         unwrap_recursive/1]).

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

phantom_tuple(_n@0_0) -> _n@0_0.

phantom_tuple_value(_value@0_0) ->
    _n@1_0 = _value@0_0,
    _n@1_0.

phantom_record(_id@0_0, _name@1_0) ->
    'Elixir.Patina.Wrappers.PhantomRecord':'$patina_constructor__PhantomRecord'(_id@0_0,
                                                                                _name@1_0).

phantom_record_name(_value@0_0) ->
    _value@0_0#'Elixir.Patina.Wrappers.PhantomRecord':'PhantomRecord'.name.

phantom_record_marker(_value@0_0) ->
    #'Elixir.Patina.Wrappers.PhantomRecord':'PhantomRecord'{id
                                                                = _,
                                                            name = _} =
        _value@0_0,
    'Elixir.Patina.Core.PhantomData'.

rename_phantom_record(_value@0_0) ->
    _value@0_0#'Elixir.Patina.Wrappers.PhantomRecord':'PhantomRecord'{name
                                                                          =
                                                                          <<"renamed"/utf8>>}.

recursive() ->
    _0@2_0 = begin
                 _0@1_0 = begin _0@0_0 = 17, _0@0_0 end, _0@1_0
             end,
    'Elixir.Patina.ForeignStructs.Recursive':'$patina_constructor__Next'(_0@2_0).

unwrap_recursive(_value@0_0) ->
    case _value@0_0 of
        _value@2_0 = _n@1_0
            when erlang:is_integer(_value@2_0) ->
            _n@1_0;
        #'Elixir.Patina.ForeignStructs.Recursive':'Next'{'0' =
                                                             _wrapper@3_0} ->
            unwrap_recursive(_wrapper@3_0)
    end.
