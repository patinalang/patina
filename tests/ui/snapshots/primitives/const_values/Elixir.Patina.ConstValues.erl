-file("tests/ui/src/primitives/const_values.pt", 1).

-module('Elixir.Patina.ConstValues').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         const_values/0,
         binary_pattern_values/0]).

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

const_values() ->
    begin
        _element@0_0 = [1, 2, 3],
        _element@2_0 = begin
                           _key@1_0 = <<"answer"/utf8>>, #{_key@1_0 => 42}
                       end,
        {_element@0_0, _element@2_0}
    end.

binary_pattern_values() ->
    begin
        _element@0_0 = <<80, 84, 2:16/integer, $ł/utf16,
                         1.5:32/float>>,
        _element@3_0 = begin
                           _element@1_0 = <<1:1/integer, 0:2/integer>>,
                           _element@2_0 = <<1:3/integer>>,
                           [_element@1_0, _element@2_0]
                       end,
        {_element@0_0, _element@3_0}
    end.
