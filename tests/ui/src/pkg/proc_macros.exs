ExUnit.start()

defmodule ProcMacrosTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.ProcMacros

  test "bang macros expand at compile time" do
    assert """
           3
           5
           7
           10 20 30
           42
           """ == capture_io(&ProcMacros.bang/0)
  end

  test "attribute macros receive and rewrite the annotated item" do
    assert """
           42 42
           """ == capture_io(&ProcMacros.attribute/0)
  end

  test "custom derives accept helper attributes and generate items" do
    assert """
           3 7 Point
           """ == capture_io(&ProcMacros.derive/0)
  end

  test "token streams round-trip through the wire format" do
    assert """
           7 20 3
           """ == capture_io(&ProcMacros.round_trip/0)
  end

  test "the token zoo survives the round trip" do
    assert ~S"""
           4 15 0.25 a "quoted" string no \escapes atom 'raw \\atom' 3 true -5
           1 2
           """ == capture_io(&ProcMacros.token_zoo/0)
  end

  test "a metavariable group forwarded to a proc macro round-trips" do
    assert """
           3
           """ == capture_io(&ProcMacros.forwarded/0)
  end

  test "reconstructing tokens through the wrapper API preserves meaning" do
    assert """
           9
           """ == capture_io(&ProcMacros.rebuilt/0)
  end

  # Like rustc, the parser flattens an invisible group from a proc macro in
  # expression position, so `<1 + 2> * 3` is 7.
  test "streams and groups built by a macro" do
    assert """
           3
           1 2 7
           """ == capture_io(&ProcMacros.built_streams/0)
  end

  test "token streams render as source text" do
    assert ~S"""
           `x` += f(1, "q\n") r#"raw"# a"at" 'c' #[z] a.b if (y) m! (1) if ! w receive (v) `if`(u) fn(t) pkg :: s `pkg` :: r
           """ == capture_io(&ProcMacros.rendered/0)
  end

  test "every typeable literal constructor produces its value" do
    assert """
           97 1 123456789012345678901234567890 0.25 s a
           """ == capture_io(&ProcMacros.constructors/0)
  end

  test "constructed literals are escaped and read back unchanged" do
    assert ~S"""
           a"b\c
           d'e 39 92 34 380 'x "y"'
           """ == capture_io(&ProcMacros.escaped/0)
  end

  test "call-site identifiers resolve in the caller" do
    assert """
           5
           """ == capture_io(&ProcMacros.call_site/0)
  end

  test "raw identifiers resolve like their unraw spelling" do
    assert """
           7
           """ == capture_io(&ProcMacros.raw_identifier/0)
  end

  test "set_span changes only the span of a token tree" do
    assert """
           5
           -4
           """ == capture_io(&ProcMacros.set_span/0)
  end
end
