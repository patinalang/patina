ExUnit.start()

defmodule ReprTransparentTest do
  use ExUnit.Case

  alias Patina.ReprTransparent
  alias Patina.ReprTransparent.NativeValue
  alias Patina.ReprTransparent.Point

  setup_all do
    Code.ensure_loaded!(NativeValue)
    :ok
  end

  test "transparent structs are represented by their payload" do
    assert 2.345 = ReprTransparent.celsius(2.345)
    assert 2.345 = ReprTransparent.degrees(2.345)
  end

  test "closed transparent variants use guarded runtime shapes" do
    assert 42 = ReprTransparent.integer(42)
    assert :integer = ReprTransparent.classify(42)

    assert %Point{x: 1, y: 2} = point = ReprTransparent.point(1, 2)
    assert :point = ReprTransparent.classify(point)
  end

  test "open transparent variants wrap only ambiguous values" do
    assert {:dynamic, "text"} = ReprTransparent.open_dynamic("text")
    assert 42 = ReprTransparent.open_dynamic(42)
    assert {:dynamic, {:tagged, 1}} = ReprTransparent.open_dynamic({:tagged, 1})
    assert {:dynamic, {:dynamic, 1}} = ReprTransparent.open_dynamic({:dynamic, 1})

    assert "text" = ReprTransparent.open_text("text")
    assert {:tagged, 1} = ReprTransparent.open_tagged(1)

    record = ReprTransparent.open_record(1)
    assert {:dynamic, ^record} = ReprTransparent.open_dynamic(record)

    # At these instantiations the payload can collide with nothing, so it always round-trips.
    # That there is no wrapping code left to run is pinned by the `.erl` snapshot.
    assert 42 = ReprTransparent.typed_int_option(42)
    assert :hi = ReprTransparent.typed_atom_option(:hi)
  end

  test "open transparent matching accepts wrapped and unwrapped alternatives" do
    assert :dynamic = ReprTransparent.classify_open({:dynamic, "text"})
    assert :dynamic = ReprTransparent.classify_open(42)
    assert :dynamic = ReprTransparent.classify_open({:dynamic, {:dynamic, 1}})
    assert :text = ReprTransparent.classify_open("text")
    assert :tagged = ReprTransparent.classify_open({:tagged, 1})
    assert :record = ReprTransparent.classify_open(ReprTransparent.open_record(1))
  end

  test "open shapes propagate through nested generic transparent enums" do
    assert 42 = ReprTransparent.nested_open(42)
    assert {:nested, "text"} = ReprTransparent.nested_open("text")
    assert {:nested, {:nested, 1}} = ReprTransparent.nested_open({:nested, 1})
    assert {:inner, 1} = ReprTransparent.nested_open({:inner, 1})

    assert :nested = ReprTransparent.classify_nested(42)
    assert :nested = ReprTransparent.classify_nested({:nested, "text"})
    assert :nested = ReprTransparent.classify_nested({:inner, {:inner, 1}})
    assert "text" = ReprTransparent.nested_text("text")
    assert :text = ReprTransparent.classify_nested("text")
  end

  test "a concretely instantiated nested enum can remain closed" do
    assert 42 = ReprTransparent.concrete_nested(42)
    assert "text" = ReprTransparent.concrete_text("text")
    assert :nested = ReprTransparent.classify_concrete(42)
    assert :text = ReprTransparent.classify_concrete("text")
  end

  test "term payloads are open-shaped" do
    default_tag = :"Elixir.Patina.ReprTransparent.TermOpen.Any"

    assert 42 = ReprTransparent.term_open(42)
    assert {^default_tag, "text"} = ReprTransparent.term_open("text")
    assert {^default_tag, {^default_tag, 1}} = ReprTransparent.term_open({default_tag, 1})
    assert "text" = ReprTransparent.term_text("text")

    assert :any = ReprTransparent.classify_term(42)
    assert :any = ReprTransparent.classify_term({default_tag, "text"})
    assert :text = ReprTransparent.classify_term("text")
  end

  test "a single-variant transparent enum is represented by its payload" do
    assert 42 = ReprTransparent.inner_open(42)
    assert {:inner, 1} = ReprTransparent.inner_open({:inner, 1})
    assert 42 = ReprTransparent.unwrap_inner(42)
    assert {:inner, 1} = ReprTransparent.unwrap_inner({:inner, 1})
    assert 42 = ReprTransparent.unwrap_inner_let(42)

    assert 7 = ReprTransparent.solo(7)
    assert 7 = ReprTransparent.unwrap_solo(7)
    assert 7 = ReprTransparent.unwrap_solo_let(7)
  end

  test "a payload that is itself wrapped stays distinguishable" do
    # An `OpenValue<str>` payload arrives already wrapped, so the outer variant has to wrap it
    # in turn - its shape includes the inner `{:dynamic, _}` wrapper, not just `str`.
    assert {:dynamic, {:dynamic, "x"}} = ReprTransparent.nested_wrapper("x")

    assert :inner_dynamic =
             ReprTransparent.classify_nested_wrapper({:dynamic, {:dynamic, "x"}})

    assert :inner_text = ReprTransparent.classify_nested_wrapper({:dynamic, "x"})
    assert :text = ReprTransparent.classify_nested_wrapper("x")
  end

  test "open transparent patterns work in every refutable position" do
    assert :dynamic = ReprTransparent.if_let_open(42)
    assert :dynamic = ReprTransparent.if_let_open({:dynamic, "text"})
    assert :other = ReprTransparent.if_let_open("text")

    assert 1 = ReprTransparent.while_let_open(42)
    assert 0 = ReprTransparent.while_let_open("text")

    assert :dynamic = ReprTransparent.let_else_open(42)
    assert :other = ReprTransparent.let_else_open("text")
  end
end
