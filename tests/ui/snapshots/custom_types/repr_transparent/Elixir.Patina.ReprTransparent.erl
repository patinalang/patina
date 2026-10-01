-file("tests/ui/src/custom_types/repr_transparent.pt",
      1).

-module('Elixir.Patina.ReprTransparent').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         celsius/1,
         degrees/1,
         integer/1,
         point/2,
         classify/1,
         open_dynamic/1,
         open_text/1,
         open_tagged/1,
         open_record/1,
         classify_open/1,
         nested_open/1,
         nested_text/1,
         classify_nested/1,
         concrete_nested/1,
         concrete_text/1,
         classify_concrete/1,
         term_open/1,
         term_text/1,
         classify_term/1,
         if_let_open/1,
         '$patina_loop_85_34'/2,
         while_let_open/1,
         let_else_open/1,
         inner_open/1,
         unwrap_inner/1,
         unwrap_inner_let/1,
         nested_wrapper/1,
         classify_nested_wrapper/1,
         typed_atom_option/1,
         typed_int_option/1,
         solo/1,
         unwrap_solo/1,
         unwrap_solo_let/1]).

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

celsius(_value@0_0) -> _value@0_0.

degrees(_value@0_0) -> _value@0_0.

integer(_value@0_0) -> _value@0_0.

point(_x@0_0, _y@1_0) ->
    ('Elixir.Patina.ReprTransparent.Point':'__struct__'())#{x
                                                                := _x@0_0,
                                                            y := _y@1_0}.

classify(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = _ when erlang:is_integer(_value@1_0) ->
            integer;
        _value@2_0 = _
            when erlang:is_map(_value@2_0) andalso
                     erlang:map_get('__struct__', _value@2_0) =:=
                         'Elixir.Patina.ReprTransparent.Point' ->
            point
    end.

open_dynamic(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= dynamic ->
            {dynamic, _value@1_0};
        _value@1_0 when erlang:is_binary(_value@1_0) ->
            {dynamic, _value@1_0};
        _value@1_0
            when erlang:is_record(_value@1_0,
                                  'Elixir.Patina.ReprTransparent.NativeValue',
                                  'NativeValue') ->
            {dynamic, _value@1_0};
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= tagged ->
            {dynamic, _value@1_0};
        _value@1_0 -> _value@1_0
    end.

open_text(_value@0_0) -> _value@0_0.

open_tagged(_value@0_0) -> {tagged, _value@0_0}.

open_record(_value@0_0) ->
    'Elixir.Patina.ReprTransparent.NativeValue':'$patina_constructor__NativeValue'(_value@0_0).

classify_open(_value@0_0) ->
    case _value@0_0 of
        {dynamic, _} -> dynamic;
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= dynamic),
                 not erlang:is_binary(_value@1_0),
                 not
                     erlang:is_record(_value@1_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= tagged) ->
            dynamic;
        _value@2_0 = _ when erlang:is_binary(_value@2_0) ->
            text;
        _value@3_0 = _
            when erlang:is_record(_value@3_0,
                                  'Elixir.Patina.ReprTransparent.NativeValue',
                                  'NativeValue') ->
            record;
        {tagged, _} -> tagged
    end.

nested_open(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso erlang:element(1, _value@1_0) =:= nested ->
            {nested, _value@1_0};
        _value@1_0 when erlang:is_binary(_value@1_0) ->
            {nested, _value@1_0};
        _value@1_0 -> _value@1_0
    end.

nested_text(_value@0_0) -> _value@0_0.

classify_nested(_value@0_0) ->
    case _value@0_0 of
        {nested, _} -> nested;
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= nested),
                 not erlang:is_binary(_value@1_0) ->
            nested;
        _value@2_0 = _ when erlang:is_binary(_value@2_0) -> text
    end.

concrete_nested(_value@0_0) -> _value@0_0.

concrete_text(_value@0_0) -> _value@0_0.

classify_concrete(_value@0_0) ->
    case _value@0_0 of
        _value@1_0 = _ when erlang:is_integer(_value@1_0) ->
            nested;
        _value@2_0 = _ when erlang:is_binary(_value@2_0) -> text
    end.

term_open(_value@0_0) ->
    case _value@0_0 of
        _value@1_0
            when (erlang:is_tuple(_value@1_0) andalso
                      erlang:tuple_size(_value@1_0) =:= 2)
                     andalso
                     erlang:element(1, _value@1_0) =:=
                         'Elixir.Patina.ReprTransparent.TermOpen.Any' ->
            {'Elixir.Patina.ReprTransparent.TermOpen.Any',
             _value@1_0};
        _value@1_0 when erlang:is_binary(_value@1_0) ->
            {'Elixir.Patina.ReprTransparent.TermOpen.Any',
             _value@1_0};
        _value@1_0 -> _value@1_0
    end.

term_text(_value@0_0) -> _value@0_0.

classify_term(_value@0_0) ->
    case _value@0_0 of
        {'Elixir.Patina.ReprTransparent.TermOpen.Any', _} ->
            any;
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso
                          erlang:element(1, _value@1_0) =:=
                              'Elixir.Patina.ReprTransparent.TermOpen.Any'),
                 not erlang:is_binary(_value@1_0) ->
            any;
        _value@2_0 = _ when erlang:is_binary(_value@2_0) -> text
    end.

if_let_open(_value@0_0) ->
    case _value@0_0 of
        {dynamic, _} -> dynamic;
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= dynamic),
                 not erlang:is_binary(_value@1_0),
                 not
                     erlang:is_record(_value@1_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= tagged) ->
            dynamic;
        _ -> other
    end.

'$patina_loop_85_34'(_current@1_0, _count@2_0) ->
    _ = begin
            case _current@1_0 of
                {dynamic, _} ->
                    _count@2_1 = _count@2_0 + 1,
                    _current@1_1 = <<"stop"/utf8>>,
                    {};
                _value@3_0 = _
                    when not
                             ((erlang:is_tuple(_value@3_0) andalso
                                   erlang:tuple_size(_value@3_0) =:= 2)
                                  andalso
                                  erlang:element(1, _value@3_0) =:= dynamic),
                         not erlang:is_binary(_value@3_0),
                         not
                             erlang:is_record(_value@3_0,
                                              'Elixir.Patina.ReprTransparent.NativeValue',
                                              'NativeValue'),
                         not
                             ((erlang:is_tuple(_value@3_0) andalso
                                   erlang:tuple_size(_value@3_0) =:= 2)
                                  andalso
                                  erlang:element(1, _value@3_0) =:= tagged) ->
                    _count@2_1 = _count@2_0 + 1,
                    _current@1_1 = <<"stop"/utf8>>,
                    {};
                _ ->
                    _branch_value@4_0 = begin
                                            '$patina_marker':break({break,
                                                                    0,
                                                                    {},
                                                                    _count@2_0,
                                                                    _current@1_0})
                                        end,
                    _count@2_1 = _count@2_0,
                    _current@1_1 = _current@1_0,
                    _branch_value@4_0
            end
        end,
    '$patina_marker':continue(_current@1_1, _count@2_1).

while_let_open(_value@0_0) ->
    _current@1_0 = _value@0_0,
    _count@2_0 = 0,
    case '$patina_loop_85_34'(_current@1_0, _count@2_0) of
        {break, 0, _loop_value@5_0, _count@2_1, _current@1_1} ->
            _loop_value@5_0
    end,
    _count@2_1.

let_else_open(_value@0_0) ->
    case _value@0_0 of
        {dynamic, _} -> {};
        _value@1_0 = _
            when not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= dynamic),
                 not erlang:is_binary(_value@1_0),
                 not
                     erlang:is_record(_value@1_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@1_0) andalso
                           erlang:tuple_size(_value@1_0) =:= 2)
                          andalso erlang:element(1, _value@1_0) =:= tagged) ->
            {};
        _ ->
            '$patina_marker':return(other),
            {}
    end,
    dynamic.

inner_open(_value@0_0) -> _value@0_0.

unwrap_inner(_value@0_0) -> _value@0_0.

unwrap_inner_let(_value@0_0) ->
    _payload@1_0 = _value@0_0,
    _payload@1_0.

nested_wrapper(_value@0_0) ->
    case case _value@0_0 of
             _value@1_0 when erlang:is_binary(_value@1_0) ->
                 {dynamic, _value@1_0};
             _value@1_0 -> _value@1_0
         end
        of
        _value@2_0
            when (erlang:is_tuple(_value@2_0) andalso
                      erlang:tuple_size(_value@2_0) =:= 2)
                     andalso erlang:element(1, _value@2_0) =:= dynamic ->
            {dynamic, _value@2_0};
        _value@2_0 when erlang:is_binary(_value@2_0) ->
            {dynamic, _value@2_0};
        _value@2_0
            when erlang:is_record(_value@2_0,
                                  'Elixir.Patina.ReprTransparent.NativeValue',
                                  'NativeValue') ->
            {dynamic, _value@2_0};
        _value@2_0
            when (erlang:is_tuple(_value@2_0) andalso
                      erlang:tuple_size(_value@2_0) =:= 2)
                     andalso erlang:element(1, _value@2_0) =:= tagged ->
            {dynamic, _value@2_0};
        _value@2_0 -> _value@2_0
    end.

classify_nested_wrapper(_value@0_0) ->
    case _value@0_0 of
        {dynamic, {dynamic, _}} -> inner_dynamic;
        {dynamic, _value@1_0 = _}
            when not erlang:is_binary(_value@1_0) ->
            inner_dynamic;
        _value@2_0 = {dynamic, _}
            when not
                     ((erlang:is_tuple(_value@2_0) andalso
                           erlang:tuple_size(_value@2_0) =:= 2)
                          andalso erlang:element(1, _value@2_0) =:= dynamic),
                 not erlang:is_binary(_value@2_0),
                 not
                     erlang:is_record(_value@2_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@2_0) andalso
                           erlang:tuple_size(_value@2_0) =:= 2)
                          andalso erlang:element(1, _value@2_0) =:= tagged) ->
            inner_dynamic;
        _value@2_0 = _value@1_0 = _
            when not erlang:is_binary(_value@1_0),
                 not
                     ((erlang:is_tuple(_value@2_0) andalso
                           erlang:tuple_size(_value@2_0) =:= 2)
                          andalso erlang:element(1, _value@2_0) =:= dynamic),
                 not erlang:is_binary(_value@2_0),
                 not
                     erlang:is_record(_value@2_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@2_0) andalso
                           erlang:tuple_size(_value@2_0) =:= 2)
                          andalso erlang:element(1, _value@2_0) =:= tagged) ->
            inner_dynamic;
        {dynamic, _value@3_0 = _}
            when erlang:is_binary(_value@3_0) ->
            inner_text;
        _value@4_0 = _value@3_0 = _
            when erlang:is_binary(_value@3_0),
                 not
                     ((erlang:is_tuple(_value@4_0) andalso
                           erlang:tuple_size(_value@4_0) =:= 2)
                          andalso erlang:element(1, _value@4_0) =:= dynamic),
                 not erlang:is_binary(_value@4_0),
                 not
                     erlang:is_record(_value@4_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@4_0) andalso
                           erlang:tuple_size(_value@4_0) =:= 2)
                          andalso erlang:element(1, _value@4_0) =:= tagged) ->
            inner_text;
        {dynamic, _} -> inner_other;
        _value@5_0 = _
            when not
                     ((erlang:is_tuple(_value@5_0) andalso
                           erlang:tuple_size(_value@5_0) =:= 2)
                          andalso erlang:element(1, _value@5_0) =:= dynamic),
                 not erlang:is_binary(_value@5_0),
                 not
                     erlang:is_record(_value@5_0,
                                      'Elixir.Patina.ReprTransparent.NativeValue',
                                      'NativeValue'),
                 not
                     ((erlang:is_tuple(_value@5_0) andalso
                           erlang:tuple_size(_value@5_0) =:= 2)
                          andalso erlang:element(1, _value@5_0) =:= tagged) ->
            inner_other;
        _value@6_0 = _ when erlang:is_binary(_value@6_0) ->
            text;
        _value@7_0 = _
            when erlang:is_record(_value@7_0,
                                  'Elixir.Patina.ReprTransparent.NativeValue',
                                  'NativeValue') ->
            record;
        {tagged, _} -> tagged
    end.

typed_atom_option(_value@0_0) -> _value@0_0.

typed_int_option(_value@0_0) -> _value@0_0.

solo(_value@0_0) -> _value@0_0.

unwrap_solo(_value@0_0) -> _value@0_0.

unwrap_solo_let(_value@0_0) ->
    _payload@1_0 = _value@0_0,
    _payload@1_0.
