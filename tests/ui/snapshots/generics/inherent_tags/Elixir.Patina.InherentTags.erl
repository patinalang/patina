-file("tests/ui/src/generics/inherent_tags.pt", 1).

-module('Elixir.Patina.InherentTags').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         int_value/1,
         str_value/1,
         name_f/0,
         int_f/2]).

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

int_value(_n@0_0) ->
    'Elixir.Patina.InherentTags.GenVal':'value<int>'(_n@0_0).

str_value(_s@0_0) ->
    'Elixir.Patina.InherentTags.GenVal':'value<str>'(_s@0_0).

name_f() ->
    'Elixir.Patina.InherentTags.GenVal':f({'Elixir.Patina.InherentTags.Name':'InherentTags::Show::$patina_witness'()},
                                          begin
                                              _gen_val@0_0 =
                                                  'Elixir.Patina.InherentTags.Name',
                                              _gen_val@0_0
                                          end).

int_f(_n@0_0, _x@1_0) ->
    'Elixir.Patina.InherentTags.GenVal':'f<int>'(_n@0_0,
                                                 _x@1_0).
