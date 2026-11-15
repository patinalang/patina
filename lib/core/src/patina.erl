-module(patina).

-export([compile/2, build/1, clean/1, print/2]).
-export_type([
    compile_option/0,
    build_option/0,
    clean_option/0,
    print_option/0,
    global_option/0,
    common_option/0,
    output_option/0,
    run_option/0,
    result/0,
    diagnostic/0,
    diagnostic_level/0,
    span/0,
    emit/0,
    dump/0,
    extern/0,
    info/0,
    printed/0,
    package/0,
    dependency/0
]).

-define(RAISE(Message),
    error(patina_error:exception(Message), none, [{error_info, #{module => patina_error}}])
).

-doc """
- **`root`** - Root of the Patina install. If not given, it's found relative to its executable.
- **`dir`** - Change to `dir` before doing anything, including resolving any paths.
""".
-type global_option() :: {root, binary()} | {dir, binary()}.

-doc """
Options for compiling a package.

- **`dump`** - Dump intermediate compiler output.
  `dump` differs from `emit` in that its outputs are diagnostics
  only, and there's no guarantees about their stability.
  Supplying `dump` causes `emit` to default to [] (nothing emitted by default).
- **`lib_dir`** - Add `DIR` to library search directories.
- **`allow_lint`** - Allow the lint `LINT`.
""".
-type common_option() ::
    {dump, dump() | [dump()]}
    | {lib_dir, DIR :: binary() | [DIR :: binary()]}
    | {allow_lint, LINT :: binary() | [LINT :: binary()]}
    | output_option().

-doc """
- **`out_dir`** - Output directory (default: `.`).
- **`incremental_dir`** - Directory for incremental compilation state
  (default: $out_dir/.patina_incremental).
""".
-type output_option() :: {out_dir, binary()} | {incremental_dir, binary()}.

-doc """
How to run `patina` executable.

- **`executable`** - The patina executable, a path or a name to look up in `PATH`
  (default: `"patina"`).
- **`env`** - See `t:env/0`.
""".
-type run_option() ::
    {executable, file:filename_all()}
    | {env, [
        {Name :: os:env_var_name() | binary(), Value :: os:env_var_value() | binary() | [] | false}
    ]}.

-type emit() :: meta | beam | app.
-type dump() :: tokens | ast | expanded | typed | eaf | abstr | erl | core0 | core.

-type result() :: {ok, [diagnostic()]} | {error, [diagnostic()]}.

-doc """
The file names in `spans` are relative to `dir`. The closing summaries ("aborting due to 1 previous
error" and the like) are diagnostics without spans, as are the errors that aren't about the
package's code.
""".
-type diagnostic() :: #{
    message := binary(),
    code := #{code := binary(), explanation := binary() | null} | null,
    level := diagnostic_level(),
    spans := [span()],
    children := [diagnostic()],
    rendered := binary() | null
}.

-type diagnostic_level() :: internal_compiler_error | error | warning | note | help | failure_note.

-type span() :: #{
    file_name := binary(),
    byte_start := non_neg_integer(),
    byte_end := non_neg_integer(),
    line_start := pos_integer(),
    line_end := pos_integer(),
    column_start := pos_integer(),
    column_end := pos_integer(),
    is_primary := boolean(),
    text := [
        #{text := binary(), highlight_start := pos_integer(), highlight_end := pos_integer()}
    ],
    label := binary() | null,
    suggested_replacement := binary() | null,
    suggestion_applicability := binary() | null,
    expansion := #{span := span(), macro_decl_name := binary(), def_site_span := span()} | null
}.

-doc """
- **`pkg_name`** - Package name (defaults to the stem of the source filename).
- **`pkg_version`** - Package version. Required with `app` in `emit`.
- **`otp_app_name`** - OTP application name.
  If not overridden, defaults to the package name.
- **`emit`** - Emit compiled output (default: `[meta, beam]`).
- **`extern`** - See `t:extern/0`.
""".
-type compile_option() ::
    {pkg_name, binary()}
    | {pkg_version, binary()}
    | {otp_app_name, binary()}
    | {emit, emit() | [emit()]}
    | {extern, extern() | [extern()]}
    | common_option()
    | global_option()
    | run_option().

-doc """
Package dependency as `{Name, Rmeta}`.
If `Rmeta` is omitted, the package is assumed to be in the library
search directories.
""".
-type extern() :: Name :: binary() | {Name :: binary(), Rmeta :: binary()}.

-doc "Compile a single source file as a package.".
-spec compile(binary(), [compile_option()]) -> result().
compile(File, Options) ->
    run([~"compile", File | encode_args(Options)] ++ server_args(), Options).

-doc """
- **`emit`** - Emit compiled output (default: `[meta, beam, app]`).
""".
-type build_option() ::
    {emit, emit() | [emit()]} | common_option() | global_option() | run_option().

-doc "Compile the package in the current directory, as described by its `Patina.toml`.".
-spec build([build_option()]) -> result().
build(Options) ->
    run([~"build" | encode_args(Options)] ++ server_args(), Options).

-doc """
- **`pkg_name`** - Clean up the package `pkg_name` instead of the one described in `Patina.toml`.
""".
-type clean_option() :: {pkg_name, binary()} | output_option() | global_option() | run_option().

-doc "Clean up the build artifacts & the incremental compilation files of `build/1`.".
-spec clean([clean_option()]) -> ok | {error, [diagnostic()]}.
clean(Options) ->
    case run([~"clean" | encode_args(Options)], Options) of
        {ok, _Diagnostics} -> ok;
        {error, _Diagnostics} = Error -> Error
    end.

-type print_option() :: global_option() | run_option().

-doc """
- **`package`** - Prints package's information, as in `Patina.toml` `[package]` section.
- **`dependencies`** - Prints package's dependencies, as in `Patina.toml` `[dependencies]` section.
- **`version`** - Prints the compiler's version.
- **`install_root`** - Prints the absolute path of the Patina install.
""".
-type info() :: package | dependencies | version | install_root.

-type printed() :: #{
    package => package(),
    dependencies => [dependency()],
    version => binary(),
    install_root => binary()
}.

-type package() :: #{
    name := binary(),
    otp_app_name := binary(),
    version := binary() | null,
    description := binary()
}.

-type dependency() :: #{
    name := binary(),
    version := binary() | null,
    source :=
        #{hex := binary()}
        | #{git := #{url := binary(), ref := binary() | null}}
        | #{path := binary()},
    otp_app_name := binary()
}.

-doc """
Print information about the package or Patina itself.

Prints the information as JSON. Every requested item gets its own top-level key.

    {ok, #{version := Version}} = patina:print([version], []).
""".
-spec print([info(), ...], [print_option()]) -> {ok, printed()} | {error, [diagnostic()]}.
print(Infos, Options) ->
    Infos1 = [atom_to_binary(Info) || Info <- Infos],
    case run([~"print", ~"--format=json" | Infos1] ++ encode_args(Options), Options) of
        {ok, [Printed]} -> {ok, Printed};
        {error, _Diagnostics} = Error -> Error
    end.

server_args() ->
    {ok, _} = patina_compile_server:ensure_started(),
    [~"--compile-server", patina_compile_server:socket_path()].

run(Args, Options) ->
    Exe = executable_path(Options),
    Env = [encode_env_var(Var) || Var <- proplists:get_value(env, Options, [])],
    Port = open_port(
        {spawn_executable, Exe},
        [
            {args, [~"--error-format", ~"json" | Args]},
            {env, Env},
            binary,
            stderr_to_stdout,
            exit_status,
            hide
        ]
    ),

    {Status, Output0} = collect_output(Port, []),
    Output = decode_lines(unicode:characters_to_binary(Output0)),
    case Status of
        0 -> {ok, Output};
        _ -> {error, Output}
    end.

collect_output(Port, Output) ->
    receive
        {Port, {data, Data}} -> collect_output(Port, [Output | Data]);
        {Port, {exit_status, Status}} -> {Status, Output}
    end.

decode_lines(Output) ->
    try
        [decode_line(Line) || Line <- binary:split(Output, ~"\n", [global, trim_all])]
    catch
        error:_ -> ?RAISE(Output)
    end.

decode_line(Line) ->
    {Value, ok, <<>>} = json:decode(Line, ok, #{object_push => fun push_key/3}),
    Value.

push_key(~"$message_type", _Value, Acc) -> Acc;
push_key(~"level", Level, Acc) -> [{level, decode_diagnostic_level(Level)} | Acc];
push_key(Key, Value, Acc) -> [{decode_key(Key), Value} | Acc].

decode_key(Key) ->
    maps:get(Key, #{
        ~"byte_end" => byte_end,
        ~"byte_start" => byte_start,
        ~"children" => children,
        ~"code" => code,
        ~"column_end" => column_end,
        ~"column_start" => column_start,
        ~"def_site_span" => def_site_span,
        ~"dependencies" => dependencies,
        ~"description" => description,
        ~"expansion" => expansion,
        ~"explanation" => explanation,
        ~"file_name" => file_name,
        ~"git" => git,
        ~"hex" => hex,
        ~"highlight_end" => highlight_end,
        ~"highlight_start" => highlight_start,
        ~"install_root" => install_root,
        ~"is_primary" => is_primary,
        ~"label" => label,
        ~"level" => level,
        ~"line_end" => line_end,
        ~"line_start" => line_start,
        ~"macro_decl_name" => macro_decl_name,
        ~"message" => message,
        ~"name" => name,
        ~"otp_app_name" => otp_app_name,
        ~"package" => package,
        ~"path" => path,
        ~"ref" => ref,
        ~"rendered" => rendered,
        ~"source" => source,
        ~"span" => span,
        ~"spans" => spans,
        ~"suggested_replacement" => suggested_replacement,
        ~"suggestion_applicability" => suggestion_applicability,
        ~"text" => text,
        ~"url" => url,
        ~"version" => version
    }).

decode_diagnostic_level(~"error: internal compiler error") -> internal_compiler_error;
decode_diagnostic_level(~"error") -> error;
decode_diagnostic_level(~"warning") -> warning;
decode_diagnostic_level(~"note") -> note;
decode_diagnostic_level(~"help") -> help;
decode_diagnostic_level(~"failure-note") -> failure_note.

encode_args(Options) ->
    lists:flatmap(fun encode_arg/1, Options).

encode_arg({executable, _Exe}) ->
    [];
encode_arg({env, _Env}) ->
    [];
encode_arg({dir, Dir}) ->
    [~"-C", Dir];
encode_arg({Key, Values}) when is_list(Values) ->
    lists:flatmap(fun(Value) -> encode_arg({Key, Value}) end, Values);
encode_arg({Key, Value}) ->
    Flag = binary:replace(atom_to_binary(Key), ~"_", ~"-", [global]),
    [<<"--", Flag/binary, "=", (encode_arg_val(Value))/binary>>].

encode_arg_val(Value) when is_atom(Value) -> atom_to_binary(Value);
encode_arg_val({Key, Value}) -> <<Key/binary, "=", Value/binary>>;
encode_arg_val(Value) when is_binary(Value) -> Value.

encode_env_var({Name0, Value0}) ->
    Name = bin_as_list(Name0),
    Value = bin_as_list(Value0),
    {Name, Value}.

executable_path(Opts) ->
    Exe = proplists:get_value(executable, Opts, "patina"),
    maybe
        [_] ?= filename:split(Exe),
        % `os:find_executable/1` does not work with binary paths.
        Name = bin_as_list(Exe, file:native_name_encoding()),
        Path = os:find_executable(Name),
        true ?= Path =/= false,

        Path
    else
        _ -> Exe
    end.

bin_as_list(Val) when is_binary(Val) -> unicode:characters_to_list(Val);
bin_as_list(Val) -> Val.

bin_as_list(Val, Encoding) when is_binary(Val) -> unicode:characters_to_list(Val, Encoding);
bin_as_list(Val, _Encoding) -> Val.
