-file("tests/ui/src/custom_types/enum/enum_use__01.pt",
      5).

-module('Elixir.Patina.EnumUse01').

-compile([no_auto_import]).

-export_record([]).

-export(['__info__'/1, main/0]).

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

main() ->
    _stage@0_0 = 'Elixir.Patina.EnumUse01.Stage.Beginner',
    _role@1_0 = 'Elixir.Patina.EnumUse01.Role.Student',
    case _stage@0_0 of
        'Elixir.Patina.EnumUse01.Stage.Beginner' ->
            io:put_chars([[<<"Beginners are starting their learning "
                             "journey!">>]]),
            io:nl();
        'Elixir.Patina.EnumUse01.Stage.Advanced' ->
            io:put_chars([[<<"Advanced learners are mastering their "
                             "subjects...">>]]),
            io:nl()
    end,
    case _role@1_0 of
        'Elixir.Patina.EnumUse01.Role.Student' ->
            io:put_chars([[<<"Students are acquiring knowledge!">>]]),
            io:nl();
        'Elixir.Patina.EnumUse01.Role.Teacher' ->
            io:put_chars([[<<"Teachers are spreading knowledge!">>]]),
            io:nl()
    end.
