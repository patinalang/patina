-file("tests/ui/src/custom_types/enum/enum_use__01.pt",
      5).

-module('Elixir.Patina.EnumUse01').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

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
            io:put_chars(begin
                             _element@2_0 =
                                 [<<"Beginners are starting their learning "
                                    "journey!"/utf8>>],
                             [_element@2_0]
                         end),
            io:nl(),
            {};
        'Elixir.Patina.EnumUse01.Stage.Advanced' ->
            io:put_chars(begin
                             _element@3_0 =
                                 [<<"Advanced learners are mastering their "
                                    "subjects..."/utf8>>],
                             [_element@3_0]
                         end),
            io:nl(),
            {}
    end,
    case _role@1_0 of
        'Elixir.Patina.EnumUse01.Role.Student' ->
            io:put_chars(begin
                             _element@4_0 =
                                 [<<"Students are acquiring knowledge!"/utf8>>],
                             [_element@4_0]
                         end),
            io:nl(),
            {};
        'Elixir.Patina.EnumUse01.Role.Teacher' ->
            io:put_chars(begin
                             _element@5_0 =
                                 [<<"Teachers are spreading knowledge!"/utf8>>],
                             [_element@5_0]
                         end),
            io:nl(),
            {}
    end.
