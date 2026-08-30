-file("tests/ui/src/primitives/atom.pt", 1).
-module('Elixir.Patina.Atom').
-compile([no_auto_import]).
-export_record([]).
-export(['__info__'/1,
         annotated/0,
         named/0,
         escaped/0,
         raw/0,
         unicode/0,
         equal/2,
         less_than/2,
         classify/1]).
'__info__'(functions) ->
    ordsets:subtract(lists:sort(module_info(exports)),
                     [{'__info__', 1},
                      {module_info, 0},
                      {module_info, 1}]);
'__info__'(macros) ->
    [];
'__info__'(struct) ->
    nil;
'__info__'(exports_md5) ->
    erlang:md5(term_to_binary({lists:sort(module_info(exports)),
                               [],
                               '__info__'(struct)},
                              [deterministic]));
'__info__'(deprecated) ->
    [];
'__info__'(Key) ->
    module_info(Key).
annotated() ->
    _value@0_0 = hello,
    _value@0_0.
named() ->
    named.
escaped() ->
    'line\nbreak'.
raw() ->
    'no \\escapes'.
unicode() ->
    'za\x{17C}ó\x{142}\x{107}'.
equal(_left@0_0, _right@1_0) ->
    _left@0_0 =:= _right@1_0.
less_than(_left@0_0, _right@1_0) ->
    _left@0_0 < _right@1_0.
classify(_value@0_0) ->
    case _value@0_0 of
        hello ->
            greeting;
        goodbye ->
            farewell;
        _ ->
            unknown
    end.
