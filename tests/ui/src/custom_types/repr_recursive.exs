ExUnit.start()

defmodule ReprRecursiveTest do
  use ExUnit.Case
  alias Patina.ReprRecursive

  setup_all do
    Code.ensure_all_loaded!([
      ReprRecursive.List,
      ReprRecursive.Tree,
      ReprRecursive.Chain,
      ReprRecursive.Left,
      ReprRecursive.Right,
      ReprRecursive.RightFirst,
      ReprRecursive.LeftLater,
      ReprRecursive.Generic,
      ReprRecursive.Growing,
      ReprRecursive.RecordChain,
      ReprRecursive.OverriddenChain,
      ReprRecursive.Outer,
      ReprRecursive.Inner,
      ReprRecursive.NativeWrapper
    ])

    :ok
  end

  test "direct recursive constructors support construction and matching" do
    assert 3 == ReprRecursive.sum(ReprRecursive.list())
    assert ReprRecursive.List == :records.get_module(ReprRecursive.list())
    assert 7 == ReprRecursive.tree_sum(ReprRecursive.tree())
    assert ReprRecursive.Tree == :records.get_module(ReprRecursive.tree())
  end

  test "self-recursive inferred newtypes become records with accessible fields" do
    chain = ReprRecursive.chain()
    assert 2 == ReprRecursive.depth(chain)
    assert ReprRecursive.Chain == :records.get_module(chain)
    assert ReprRecursive.Chain == :records.get_module(:records.get(:"0", chain))
  end

  test "mutually recursive enums become records" do
    left = ReprRecursive.mutual()
    assert 2 == ReprRecursive.mutual_depth(left)
    assert ReprRecursive.Left == :records.get_module(left)
    assert ReprRecursive.Right == :records.get_module(:records.get(:"0", left))
  end

  test "reversed declaration order preserves mutually recursive records" do
    right = ReprRecursive.reverse_mutual()
    assert ReprRecursive.RightFirst == :records.get_module(right)
    assert ReprRecursive.LeftLater == :records.get_module(:records.get(:"0", right))
  end

  test "mutually recursive structs and enums become records" do
    outer = ReprRecursive.struct_cycle()
    assert ReprRecursive.Outer == :records.get_module(outer)
    assert ReprRecursive.Inner == :records.get_module(:records.get(:"0", outer))
    assert 1 == ReprRecursive.struct_depth(outer)
  end

  test "generic transparency forwarding detects the recursive caller" do
    value = ReprRecursive.generic()
    assert ReprRecursive.Generic == :records.get_module(value)
    assert 9 == :records.get(:"0", value)
    assert 9 == ReprRecursive.generic_value(value)
  end

  test "growing generic arguments retain the recursive record representation" do
    growing = ReprRecursive.growing()
    assert ReprRecursive.Growing == :records.get_module(growing)
    assert [2] == ReprRecursive.growing_field(growing)
  end

  test "tuple payloads break transparent recursion" do
    tuple = ReprRecursive.tuple_boundary()
    assert {5, ReprRecursive.TupleChain.End} == tuple
    assert 5 == ReprRecursive.tuple_value(tuple)
    assert {6, ReprRecursive.WrappedTuple.End} == ReprRecursive.wrapped_tuple()
  end

  test "list, map, and function payloads break transparent recursion" do
    assert [[]] == ReprRecursive.list_boundary()
    assert %{child: %{}} == ReprRecursive.map_boundary()
    assert is_function(ReprRecursive.function_boundary(), 0)
    assert is_function(ReprRecursive.call_boundary(ReprRecursive.function_boundary()), 0)
  end

  test "explicit tagged and native-record representations break transparent recursion" do
    assert {:wrapped, ReprRecursive.FixedGeneric.End} == ReprRecursive.fixed_generic()
    assert ReprRecursive.NativeWrapper == :records.get_module(ReprRecursive.fixed_record_generic())
    tagged = ReprRecursive.tagged()
    assert {:next, ReprRecursive.TaggedChain.End} == tagged
    assert 1 == ReprRecursive.tagged_depth(tagged)
    record = ReprRecursive.record()
    assert ReprRecursive.RecordChain == :records.get_module(record)
    assert 1 == ReprRecursive.record_depth(record)
  end

  test "variant representations override an enum's transparent attribute" do
    overridden = ReprRecursive.overridden()
    assert ReprRecursive.OverriddenChain == :records.get_module(overridden)
    assert 1 == ReprRecursive.overridden_depth(overridden)
  end
end
