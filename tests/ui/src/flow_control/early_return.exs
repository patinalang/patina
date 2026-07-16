ExUnit.start()

defmodule EarlyReturnTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.EarlyReturn

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

  test "returning operands preserve left-to-right evaluation" do
    assert """
           1
           2
           """ == capture_io(fn -> assert EarlyReturn.expression_value(true) == 2 end)

    assert """
           1
           3
           """ == capture_io(fn -> assert EarlyReturn.expression_value(false) == 4 end)
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
end
