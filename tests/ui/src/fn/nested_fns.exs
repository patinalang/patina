ExUnit.start()

defmodule NestedFnsTest do
  use ExUnit.Case

  alias Patina.NestedFns

  test "a nested fn can be used before its declaration" do
    assert NestedFns.before_declaration() == 7
  end

  test "nested fns can call each other" do
    assert NestedFns.mutual(10) == true
    assert NestedFns.mutual(7) == false
  end

  test "nested fns with the same name in different bodies stay apart" do
    assert NestedFns.first() == 1
    assert NestedFns.second() == 2
    assert NestedFns.shadowed_blocks() == 30
  end

  test "a nested fn can recurse from inside a loop" do
    assert NestedFns.recursive_loop(5) == 8
  end

  test "nested fns can be generic, declared in closures and used as values" do
    assert NestedFns.generic() == "int"
    assert NestedFns.in_closure() == 42
    assert NestedFns.as_value() == 12
  end

  test "nested fns are private unless another module calls them" do
    assert NestedFns.not_exported() == 5
    assert NestedFns.from_impl() == 101

    exports = NestedFns.module_info(:exports)
    refute {:"not_exported::hidden", 0} in exports
    assert {:"from_impl::base", 0} in exports
  end

  test "nested traits get their default bodies" do
    assert NestedFns.nested_trait() == 42
    assert NestedFns.trait_in_method() == 1000
  end

  test "nested traits with the same name in different bodies stay apart" do
    assert NestedFns.first_answer() == 1
    assert NestedFns.second_answer() == 2
    assert NestedFns.answers_in_blocks() == 34
  end
end
