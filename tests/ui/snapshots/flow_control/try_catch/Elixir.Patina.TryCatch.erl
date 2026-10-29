-file("tests/ui/src/flow_control/try_catch.pt", 1).

-module('Elixir.Patina.TryCatch').

-compile([no_auto_import]).

-import(erlang, [is_record/3]).

-export_record([]).

-export(['__info__'/1,
         error_reason/1,
         error_with_stack/1,
         argument_error/1,
         argument_error_stack/1,
         thrown/1,
         thrown_with_stack/1,
         exited/1,
         elixir_name/1,
         elixir_reason/1,
         elixir_stack/1,
         any_class/1,
         any_with_stack/1,
         any_atom_reason/1,
         any_error_value/1,
         any_pinned_class/2,
         any_aliased_class/1,
         any_class_alternative/1,
         atom_reason/1,
         badmatch_value/1,
         map_reason/1,
         arithmetic_reason/1,
         guarded_reason/1,
         guarded_stack/1,
         unsafe_reason/1,
         catch_only/0,
         after_only/1,
         after_error/2,
         contextual_after/0]).

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

error_reason(_f@0_0) ->
    try _f@0_0() catch error:_reason@1_0 -> _reason@1_0 end.

error_with_stack(_f@0_0) ->
    try _f@0_0() catch
        error:_:_stacktrace@1_0 -> _stacktrace@1_0
    end.

argument_error(_f@0_0) ->
    try _f@0_0(), false catch error:badarg -> true end.

argument_error_stack(_f@0_0) ->
    try _f@0_0() catch
        error:badarg:_stacktrace@1_0 -> _stacktrace@1_0
    end.

thrown(_f@0_0) ->
    try _f@0_0() catch _value@1_0 -> _value@1_0 end.

thrown_with_stack(_f@0_0) ->
    try _f@0_0() catch _value@1_0 -> _value@1_0 end.

exited(_f@0_0) ->
    try _f@0_0() catch exit:_reason@1_0 -> _reason@1_0 end.

elixir_name(_f@0_0) ->
    try _f@0_0(), none catch
        error:(#{'__struct__' := _name@1_0}) -> _name@1_0
    end.

elixir_reason(_f@0_0) ->
    try _f@0_0(), none catch
        error:(_reason@1_0 = #{'__exception__' := true}) ->
            _reason@1_0
    end.

elixir_stack(_f@0_0) ->
    try _f@0_0() catch
        error:(#{'__exception__' := true}):_stacktrace@1_0 ->
            _stacktrace@1_0
    end.

any_class(_f@0_0) ->
    try _f@0_0(), none catch _class@1_0:_ -> _class@1_0 end.

any_with_stack(_f@0_0) ->
    try _f@0_0() catch
        _:_:_stacktrace@1_0 -> _stacktrace@1_0
    end.

any_atom_reason(_f@0_0) ->
    try _f@0_0(), {none, none} catch
        _class@1_0:_value@2_0 when erlang:is_atom(_value@2_0) ->
            {_class@1_0, _value@2_0}
    end.

any_error_value(_f@0_0) ->
    try _f@0_0() catch error:_value@1_0 -> _value@1_0 end.

any_pinned_class(_expected@0_0, _f@1_0) ->
    try _f@1_0() catch
        _expected@0_0:_value@2_0 -> _value@2_0
    end.

any_aliased_class(_f@0_0) ->
    try _f@0_0(), none catch
        _class@1_0:_ when _class@1_0 =:= error -> _class@1_0
    end.

any_class_alternative(_f@0_0) ->
    try _f@0_0() catch
        error:_value@1_0 -> _value@1_0;
        _value@1_0 -> _value@1_0
    end.

atom_reason(_f@0_0) ->
    try _f@0_0(), none catch
        error:_name@1_0 when erlang:is_atom(_name@1_0) ->
            _name@1_0
    end.

badmatch_value(_f@0_0) ->
    try _f@0_0() catch
        error:{badmatch, _value@1_0} -> _value@1_0
    end.

map_reason(_f@0_0) ->
    try _f@0_0() catch
        error:(#{kind := _value@1_0}) -> _value@1_0
    end.

arithmetic_reason(_f@0_0) ->
    try _f@0_0(), none catch
        error:badarg -> arithmetic;
        error:badarith -> arithmetic
    end.

guarded_reason(_f@0_0) ->
    try _f@0_0(), none catch
        error:_name@1_0
            when erlang:is_atom(_name@1_0), _name@1_0 =:= wanted ->
            _name@1_0
    end.

guarded_stack(_f@0_0) ->
    try _f@0_0() catch
        error:_name@2_0:_stacktrace@1_0
            when erlang:is_atom(_name@2_0), _name@2_0 =:= wanted ->
            _stacktrace@1_0
    end.

unsafe_reason(_f@0_0) ->
    try _f@0_0(), 0 catch
        {my_error, _code@1_0}
            when erlang:is_integer(_code@1_0) ->
            _code@1_0
    end.

catch_only() -> try 7 catch error:_ -> 0 end.

after_only(_recipient@0_0) ->
    try 8 after
        erlang:send(_recipient@0_0, after_success), {}
    end.

after_error(_recipient@0_0, _f@1_0) ->
    try _f@1_0() catch
        error:_reason@2_0 -> _reason@2_0
    after
        erlang:send(_recipient@0_0, after_error), {}
    end.

contextual_after() ->
    _after@0_0 = 9,
    _after@0_0.
