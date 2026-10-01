ExUnit.start()

defmodule ReprPatinaDefaultTest do
  use ExUnit.Case

  alias Patina.ReprPatinaDefault
  alias Patina.ReprPatinaDefault.{
    ExplicitAtom,
    Greedy,
    InheritedAtom,
    Large,
    LaterExplicit,
    MultipleOpen,
    Override,
    Record,
    ReservedWrapper,
    Small,
    WrapperConflict
  }


  test "fieldless structs support atom representation" do
    assert {ReprPatinaDefault.AtomUnit, ReprPatinaDefault.AtomBraced,
            ReprPatinaDefault.AtomTuple} == ReprPatinaDefault.atom_structs()
  end

  test "fieldless variants inherit atom representation" do
    assert {InheritedAtom.Unit, InheritedAtom.Braced, InheritedAtom.Tuple} ==
             ReprPatinaDefault.inherited_atoms()
  end

  test "fieldless variants support explicit atom representation" do
    assert {ExplicitAtom.Unit, ExplicitAtom.Braced, ExplicitAtom.Tuple} ==
             ReprPatinaDefault.explicit_atoms()
  end

  test "fieldless constructor forms support pattern matching" do
    assert {true, true, true} == ReprPatinaDefault.atom_patterns()
  end

  test "closed newtypes claim transparent shapes from the top" do
    assert 1 = ReprPatinaDefault.greedy_first(1)
    assert "text" = ReprPatinaDefault.greedy_text("text")
    assert :"Elixir.Patina.ReprPatinaDefault.Greedy.Nil" = ReprPatinaDefault.greedy_nil()

    second = ReprPatinaDefault.greedy_second(2)
    assert Greedy = :records.get_module(second)
    assert :Second = :records.get_name(second)
    assert 2 = :records.get(:"0", second)
  end

  test "one open variant in a small enum wraps dynamically" do
    tag = :"Elixir.Patina.ReprPatinaDefault.Small.Open"
    nil_tag = :"Elixir.Patina.ReprPatinaDefault.Small.Nil"

    assert 42 = ReprPatinaDefault.small_open(42)
    assert {^tag, "text"} = ReprPatinaDefault.small_open("text")
    assert {^tag, ^nil_tag} = ReprPatinaDefault.small_open(nil_tag)
    assert {^tag, {^tag, 1}} = ReprPatinaDefault.small_open({tag, 1})
    assert "text" = ReprPatinaDefault.small_text("text")
    assert ^nil_tag = ReprPatinaDefault.small_nil()

    assert :open = ReprPatinaDefault.classify_small(42)
    assert :open = ReprPatinaDefault.classify_small({tag, "text"})
    assert :text = ReprPatinaDefault.classify_small("text")
    assert ReprPatinaDefault.classify_small(nil_tag) == nil
  end

  test "open variants use records beyond the dynamic-wrap cutoff" do
    open = ReprPatinaDefault.large_open(:value)
    assert Large = :records.get_module(open)
    assert :Open = :records.get_name(open)
    assert :value = :records.get(:"0", open)
    assert "text" = ReprPatinaDefault.large_text("text")
  end

  test "the first open variant is transparent and later ones use records" do
    assert 1 = ReprPatinaDefault.multiple_first(1)

    second = ReprPatinaDefault.multiple_second("second")
    assert MultipleOpen = :records.get_module(second)
    assert :Second = :records.get_name(second)
    assert "second" = :records.get(:"0", second)

    assert "text" = ReprPatinaDefault.multiple_text("text")
  end

  test "a later explicit representation reserves its shape before inferred newtypes" do
    inferred = ReprPatinaDefault.later_inferred(7)
    assert LaterExplicit == :records.get_module(inferred)
    assert :Inferred == :records.get_name(inferred)
    assert 7 == :records.get(:"0", inferred)
    assert 7 == ReprPatinaDefault.later_explicit(7)
  end

  test "a later open variant reserves its wrapper before closed newtypes" do
    pair = ReprPatinaDefault.wrapper_pair({:open, 7})
    assert WrapperConflict == :records.get_module(pair)
    assert :Pair == :records.get_name(pair)
    assert {:open, 7} == :records.get(:"0", pair)
    assert {:open, {:open, 7}} == ReprPatinaDefault.wrapper_open({:open, 7})
    assert 7 == ReprPatinaDefault.wrapper_open(7)
    assert :pair == ReprPatinaDefault.classify_wrapper(pair)
    assert :open == ReprPatinaDefault.classify_wrapper({:open, {:open, 7}})
  end

  test "an open variant uses a record when its wrapper is already reserved" do
    assert {:open, 7} == ReprPatinaDefault.reserved_wrapper_reserved(7)

    open = ReprPatinaDefault.reserved_wrapper_open(7)
    assert ReservedWrapper == :records.get_module(open)
    assert :open == :records.get_name(open)
    assert 7 == :records.get(:"0", open)

    assert 7 == ReprPatinaDefault.reserved_wrapper_available(7)
    assert {:later, {:open, 7}} ==
             ReprPatinaDefault.reserved_wrapper_available({:open, 7})
  end

  test "Patina overrides an inherited representation" do
    assert 7 == ReprPatinaDefault.override_inferred(7)

    record = ReprPatinaDefault.override_record("text")
    assert Override == :records.get_module(record)
    assert :Record == :records.get_name(record)
    assert "text" == :records.get(:"0", record)
  end

  test "structs use the Patina default too" do
    assert :"Elixir.Patina.ReprPatinaDefault.Unit" = ReprPatinaDefault.unit()
    assert 7 = ReprPatinaDefault.newtype(7)

    record = ReprPatinaDefault.record(7, "seven")
    assert Record = :records.get_module(record)
    assert :Record = :records.get_name(record)
    assert 7 = :records.get(:value, record)
    assert "seven" = :records.get(:label, record)
    assert 7 = ReprPatinaDefault.record_value(record)
  end
end
