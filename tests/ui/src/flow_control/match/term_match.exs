ExUnit.start()

defmodule TermMatchTest do
  use ExUnit.Case

  alias Patina.TermMatch
  alias Patina.TermMatch.ElixirPoint
  alias Patina.TermMatch.Point

  setup_all do
    Code.ensure_all_loaded!([Point, ElixirPoint])
  end

  test "safe term patterns destructure BEAM values" do
    assert TermMatch.literals(nil) == 1
    assert TermMatch.literals(1) == 2
    assert TermMatch.literals(1.5) == 3
    assert TermMatch.literals("hello") == 4
    assert TermMatch.literals(true) == 5
    assert TermMatch.literals(42) == 6
    assert TermMatch.literals(:other) == 0

    assert TermMatch.tuple({:some, {1, 9}}) == 9
    assert TermMatch.tuple({:some, {1, :not_an_int}}) == 0
    assert TermMatch.list_match([1, 2, 3]) == {1, [2, 3]}
    assert TermMatch.map(%{"k" => 7}) == 7
  end

  test "pins, guards, let patterns, and closures work on term" do
    assert TermMatch.pinned(7, 7)
    refute TermMatch.pinned(8, 7)
    assert TermMatch.guarded(42)
    refute TermMatch.guarded(41)
    assert TermMatch.inferred_class_scrutinee()
    assert TermMatch.if_let_term({:ok, 1})
    refute TermMatch.if_let_term(:ok)
    assert TermMatch.let_else_term([1])
    refute TermMatch.let_else_term(:not_a_list)
    assert TermMatch.closure_term([1])
    refute TermMatch.closure_term({1})
  end

  test "or-patterns work at and below the term boundary" do
    assert TermMatch.literal_or(1)
    assert TermMatch.literal_or(2)
    assert TermMatch.literal_or(:ok)
    refute TermMatch.literal_or(3)
    refute TermMatch.literal_or(:error)

    assert TermMatch.nested_or({:ok, 1})
    assert TermMatch.nested_or({:ok, 2})
    assert TermMatch.nested_or({:error, 1})
    assert TermMatch.nested_or({:error, 2})
    refute TermMatch.nested_or({:other, 1})
    refute TermMatch.nested_or({:ok, 3})
    refute TermMatch.nested_or({:ok, :not_an_int})
  end

  test "class patterns test and bind their BEAM classes" do
    assert TermMatch.int_class(41) == 42
    assert TermMatch.int_class(41.0) == 0
    assert TermMatch.float_class(2.5) == 3.5
    assert TermMatch.atom_class(:ok) == :ok
    assert TermMatch.list_class([1])
    assert TermMatch.map_class(%{"k" => 1})
    assert TermMatch.tuple_class({1, 2})
    assert TermMatch.fn_class(fn -> :ok end)
    assert TermMatch.bits_class(<<1::1>>)
    assert TermMatch.bin_class(<<1>>)
    assert TermMatch.pid_class(self())
    assert TermMatch.ref_class(make_ref())
  end

  test "class patterns compose with structural patterns" do
    assert TermMatch.number(1)
    assert TermMatch.number(1.0)
    assert TermMatch.alias_list([1, 2]) == {[1, 2], 1, [2]}
    assert TermMatch.nested({1, :ok})
    assert TermMatch.nested_list([1, 2])
    assert TermMatch.nested_map(%{"k" => 1.5})
    assert TermMatch.tuple_terms({1, :ok})
    assert TermMatch.struct_term({:wrapper, 1})
    refute TermMatch.struct_term({:wrapper, "one"})
  end

  test "transparent enums match by runtime representation" do
    assert TermMatch.option_none(nil) == 1
    assert TermMatch.option_some(41) == 41
    assert TermMatch.inferred_some(41) == 42
    assert TermMatch.nested_some(41) == 42
    assert TermMatch.option_some("not an int") == 0
  end

  test "native-record and transparent structs enforce field shapes" do
    point = :records.create(Point, :Point, [x: 2, y: 3], %{is_exported: true})
    wrong_x = :records.create(Point, :Point, [x: "two", y: 3], %{is_exported: true})
    wrong_y = :records.create(Point, :Point, [x: 2, y: "three"], %{is_exported: true})

    assert TermMatch.point(point) == 5
    assert TermMatch.point_rest(point) == 2
    assert TermMatch.point(wrong_x) == 0
    assert TermMatch.point_rest(wrong_x) == 0
    assert TermMatch.point_rest(wrong_y) == 0
    assert TermMatch.point_range(point)
    refute TermMatch.point_range(wrong_x)
    refute TermMatch.point_range(wrong_y)
    assert TermMatch.celsius(2.5) == 2.5
    assert TermMatch.celsius_any(2.5)
    refute TermMatch.celsius_any(2)
    assert TermMatch.nested_in_tuple({:point, point}) == 5
  end

  test "or-patterns can mix structural and representation matching" do
    matching_point = :records.create(Point, :Point, [x: 0, y: 3], %{is_exported: true})
    other_point = :records.create(Point, :Point, [x: 1, y: 3], %{is_exported: true})
    malformed_point = :records.create(Point, :Point, [x: 0, y: "three"], %{is_exported: true})

    assert TermMatch.literal_or_point(0)
    assert TermMatch.literal_or_point(matching_point)
    refute TermMatch.literal_or_point(1)
    refute TermMatch.literal_or_point(other_point)
    refute TermMatch.literal_or_point(malformed_point)
  end

  test "tagged, Elixir, and nested term fields match their representations" do
    assert TermMatch.tagged({:tagged, 7, "seven"}) == 7
    assert TermMatch.tagged({:tagged, 7, :seven}) == 0
    assert TermMatch.elixir(%{__struct__: ElixirPoint, x: 2, y: 3}) == 5
    assert TermMatch.elixir(%{__struct__: ElixirPoint, x: "two", y: 3}) == 0
    assert TermMatch.elixir_rest(%{__struct__: ElixirPoint, x: 2, y: 3}) == 2
    assert TermMatch.elixir_rest(%{__struct__: ElixirPoint, x: "two", y: 3}) == 0
    assert TermMatch.elixir_rest(%{__struct__: ElixirPoint, x: 2, y: "three"}) == 0
    assert TermMatch.wrapped_term({:wrapper, {1, :ok}})
    refute TermMatch.wrapped_term({:wrapper, {"one", :ok}})
    assert TermMatch.wrapped_int({:wrapper, 1})
    refute TermMatch.wrapped_int({:wrapper, "one"})
  end
end
