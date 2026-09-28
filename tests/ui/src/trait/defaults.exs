ExUnit.start()

defmodule DefaultsTest do
  use ExUnit.Case

  alias Patina.Defaults

  describe "default methods" do
    test "run through the impl's delegation stub" do
      assert {"Dolly", "baaaaah"} == Defaults.sheep_calls_required("Dolly")
    end

    test "are replaced by an override" do
      assert {"Rex", "barks"} == Defaults.dog_calls_required("Rex")
    end

    test "take witnesses for their own bounded parameters" do
      assert {"Dolly", "baaaaah", "a bell"} == Defaults.sheep_with_bounded_param("Dolly")
      assert {"Rex", "woof", "a bell"} == Defaults.dog_with_bounded_param("Rex")
    end

    test "are called locally from the impl's module" do
      assert {{"Dolly", "baaaaah"}, "Dolly"} == Defaults.sheep_calls_stubs_locally("Dolly")
    end

    test "return Self" do
      assert "Dolly" == Defaults.sheep_returns_self("Dolly")
      assert "Rex" == Defaults.dog_returns_self("Rex")
    end

    test "dispatch through the receiver's witness" do
      talk = {"Dolly", "baaaaah"}
      assert {talk, {talk, talk}} == Defaults.sheep_via("Dolly")
    end

    test "call an overridden method" do
      bark = {"Rex", "barks"}
      assert {bark, {bark, bark}} == Defaults.dog_via("Rex")
    end

    test "call private helpers of the trait's module" do
      assert 42 == Defaults.double(21)
    end

    test "call supertrait methods" do
      assert {"Hello", "Ann"} == Defaults.greet("Ann")
    end

    test "call supertrait methods through dispatch" do
      assert {"Hello", "Bob"} == Defaults.greet_through_witness("Bob")
    end

    test "return impl Trait" do
      assert {} == Defaults.make()
    end
  end

  describe "defaults of a generic trait" do
    test "use the trait argument's witness" do
      assert 21 == Defaults.combine_coin_with_note(1, 2)
    end

    test "run for a generic impl" do
      assert 5 == Defaults.combine_coin_piles(1, 2)
    end

    test "dispatch through the receiver's witness" do
      assert 32 == Defaults.combine_note_piles_via(1, 2)
      assert 43 == Defaults.combine_coin_with_note_via(3, 4)
    end
  end

  describe "associated type witnesses" do
    test "of a concrete type" do
      assert "a" == Defaults.describe_boxed("a")
    end

    test "of a bounded parameter" do
      assert "b" == Defaults.describe_wrapped("b")
    end

    test "of a concrete projection" do
      assert "c" == Defaults.describe_rewrapped("c")
    end

    test "of an unbounded parameter" do
      assert "d" == Defaults.describe_loose("d")
    end

    test "in a generic function" do
      assert "e" == Defaults.describe_first_loose("e")
    end

    test "of a nested projection" do
      assert "f" == Defaults.describe_nested("f")
    end
  end

  describe "associated consts" do
    test "use the trait's default" do
      assert {1, 100} == Defaults.one_consts()
    end

    test "are overridden by the impl" do
      assert {2, 200} == Defaults.two_consts()
    end

    test "depend on the impl's parameter" do
      assert {11, 100} == Defaults.plus_one_consts()
    end

    test "dispatch through a type parameter" do
      assert 202 == Defaults.sum_two()
    end

    test "resolve statically" do
      assert 1 == Defaults.one_c()
      assert 100 == Defaults.plus_two_d()
    end

    test "are wrapped by an inherent const used from another module" do
      assert 14 == Defaults.doubled_k()
    end
  end

  describe "ne" do
    test "is overridden by primitives" do
      assert Defaults.int_ne(1, 2)
      refute Defaults.int_ne(1, 1)
      assert Defaults.str_ne("a", "b")
    end

    test "defaults to the negation of eq" do
      assert Defaults.p_ne(1, 2)
      refute Defaults.p_ne(1, 1)
    end

    test "dispatches through a type parameter" do
      refute Defaults.p_differ(3, 3)
      assert Defaults.int_differ(3, 4)
    end
  end

  describe "an override with fewer bounds than the trait method" do
    test "takes the witnesses callers pass through dispatch" do
      assert "a bell" == Defaults.ring_loud()
      assert "..." == Defaults.ring_quiet()
    end

    test "takes the witnesses callers pass directly" do
      assert "..." == Defaults.ring_quiet_directly()
    end
  end

  describe "function items with witnesses" do
    test "pass the witnesses of a generic function" do
      assert "g" == Defaults.describe_through_pointer("g")
    end

    test "call a default through the impl's stub" do
      assert Defaults.ne_through_pointer(1, 2)
    end

    test "dispatch through a type parameter" do
      assert {"Shaun", "baaaaah"} == Defaults.sheep_calls_required_through_pointer("Shaun")
    end
  end
end
