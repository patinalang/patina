ExUnit.start()

defmodule EarlyReturnTest do
  use ExUnit.Case

  alias Patina.EarlyReturn

  setup do
    Process.put(:marks, [])
    :ok
  end

  test "explicit returns leave the function" do
    assert EarlyReturn.is_negative(1) == false
    assert EarlyReturn.is_negative(-1) == true
  end

  test "an early return and an implicit tail produce their ordinary values" do
    assert EarlyReturn.is_negative_implicit(1) == false
    assert EarlyReturn.is_negative_implicit(-1) == true
  end

  test "returns in closures do not affect their enclosing function" do
    assert EarlyReturn.closure_value(true) == 10
    assert EarlyReturn.closure_value(false) == 10
  end

  test "returns in an inlined closure return from the closure" do
    assert EarlyReturn.inlined_closure_value(true) == 40
    assert EarlyReturn.inlined_closure_value(false) == 50
  end

  test "returns in a closure passed as an argument return from the closure" do
    assert EarlyReturn.passed_closure_value(-1) == 0
    assert EarlyReturn.passed_closure_value(2) == 30
  end

  test "returning operands preserve left-to-right evaluation" do
    assert EarlyReturn.expression_value(true) == 2
    assert Process.put(:marks, []) == [2, 1]

    assert EarlyReturn.expression_value(false) == 4
    assert Process.put(:marks, []) == [3, 1]
  end

  test "returns propagate out of match arms" do
    assert EarlyReturn.match_stopped() == 10
    assert EarlyReturn.match_continued() == 21
  end

  test "returns propagate through nested value-producing blocks" do
    assert EarlyReturn.nested_value(0) == 10
    assert EarlyReturn.nested_value(1) == 20
    assert EarlyReturn.nested_value(2) == 31
  end

  test "returns propagate out of a match scrutinee" do
    assert EarlyReturn.match_scrutinee_value(true) == 40
    assert EarlyReturn.match_scrutinee_value(false) == 60
  end

  test "returns propagate out of an if condition" do
    assert EarlyReturn.if_condition_value(true) == 10
    assert EarlyReturn.if_condition_value(false) == 20
  end

  test "returns from a try body run the after body" do
    assert EarlyReturn.return_in_try(true, fn -> :ok end) == 10
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_try(false, fn -> :ok end) == 21
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_try(false, fn -> :erlang.error(:boom) end) == 31
    assert Process.put(:marks, []) == [0]
  end

  test "returns from a catch arm run the after body" do
    assert EarlyReturn.return_in_catch(true, fn -> :erlang.error(:boom) end) == 10
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_catch(false, fn -> :erlang.error(:boom) end) == 31
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_catch(true, fn -> :ok end) == 21
    assert Process.put(:marks, []) == [0]
  end

  test "returns from an after body replace the try's value and any uncaught exception" do
    assert EarlyReturn.return_in_after(true, fn -> :ok end) == 10
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_after(true, fn -> :erlang.error(:boom) end) == 10
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_after(true, fn -> throw(:boom) end) == 10
    assert Process.put(:marks, []) == [0]

    assert EarlyReturn.return_in_after(false, fn -> :ok end) == 21
    assert Process.put(:marks, []) == [0]

    assert catch_throw(EarlyReturn.return_in_after(false, fn -> throw(:boom) end)) == :boom
    assert Process.put(:marks, []) == [0]
  end

  test "a return from an after body overrides a return from the try body" do
    assert EarlyReturn.return_in_try_and_after(true, true) == 3
    assert Process.put(:marks, []) == [3, 1]

    assert EarlyReturn.return_in_try_and_after(true, false) == 1
    assert Process.put(:marks, []) == [1]

    assert EarlyReturn.return_in_try_and_after(false, true) == 3
    assert Process.put(:marks, []) == [3]

    assert EarlyReturn.return_in_try_and_after(false, false) == 2
    assert Process.put(:marks, []) == []
  end

  test "returns run the after bodies of all enclosing trys, innermost first" do
    assert EarlyReturn.return_through_nested_afters(true) == 1
    assert Process.put(:marks, []) == [20, 10, 1]

    assert EarlyReturn.return_through_nested_afters(false) == 2
    assert Process.put(:marks, []) == [20, 10]
  end

  test "returns from an inner after body run the outer after body" do
    assert EarlyReturn.return_from_inner_after(true) == 1
    assert Process.put(:marks, []) == [20]

    assert EarlyReturn.return_from_inner_after(false) == 2
    assert Process.put(:marks, []) == [20, 10]
  end

  test "returns leave a try without an after body" do
    assert EarlyReturn.return_in_try_without_after(true, fn -> :ok end) == 10
    assert EarlyReturn.return_in_try_without_after(false, fn -> :erlang.error(:boom) end) == 31
  end

  test "returns from a large after body leave the enclosing function" do
    assert EarlyReturn.return_in_large_after(true, 1) == 25
    assert Process.put(:marks, []) == [1]

    assert EarlyReturn.return_in_large_after(false, 1) == 2
    assert Process.put(:marks, []) == [25, 1]
  end
end
