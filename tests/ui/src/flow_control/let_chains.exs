ExUnit.start()

defmodule LetChainsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.LetChains

  test "a let followed by a boolean operand needs both to hold" do
    assert LetChains.let_and_bool(-1) == 0
    assert LetChains.let_and_bool(3) == 0
    assert LetChains.let_and_bool(7) == 7
  end

  test "a boolean followed by a let binds only when the boolean holds" do
    assert LetChains.bool_and_let(false, 3) == -1
    assert LetChains.bool_and_let(true, -1) == -1
    assert LetChains.bool_and_let(true, 3) == 3
  end

  test "later operands see the bindings of earlier lets" do
    assert LetChains.let_let_cond(-1, 1) == 0
    assert LetChains.let_let_cond(5, -1) == 0
    assert LetChains.let_let_cond(1, 5) == 0
    assert LetChains.let_let_cond(5, 1) == 6
  end

  test "a parenthesized `&&` chain is part of the let chain" do
    assert LetChains.let_and_nested(-1, true, true) == 0
    assert LetChains.let_and_nested(4, true, false) == 0
    assert LetChains.let_and_nested(4, true, true) == 4
  end

  test "an or-pattern operand binds from either alternative" do
    assert LetChains.or_pattern(0, true) == 10
    assert LetChains.or_pattern(1, true) == 20
    assert LetChains.or_pattern(2, true) == 0
    assert LetChains.or_pattern(0, false) == 0
  end

  test "an assignment in the first operand reaches then and else" do
    assert LetChains.assign_in_first(-1) == 5
    assert LetChains.assign_in_first(3) == 8
  end

  test "assignments in later operands reach else and the code after the if" do
    assert LetChains.assign_in_later(-1) == 0
    assert LetChains.assign_in_later(0) == 1001
    assert LetChains.assign_in_later(3) == 11011
    assert LetChains.assign_in_later(10) == 1_000_011
  end

  test "variables assigned in different operands reach else with their latest values" do
    assert LetChains.assign_two_vars(-1) == 0
    assert LetChains.assign_two_vars(1) == 10
    assert LetChains.assign_two_vars(3) == 36
    assert LetChains.assign_two_vars(10) == 30
  end

  test "a variable local to an operand doesn't leak out of it" do
    assert LetChains.local_in_operand(2) == -1
    assert LetChains.local_in_operand(5) == 5
  end

  test "then and else rebind an outer variable" do
    assert LetChains.mutate_in_branches(-1) == 0
    assert LetChains.mutate_in_branches(4) == 5
  end

  test "then and else can initialize the same outer binding" do
    assert LetChains.deferred_init(-1) == 0
    assert LetChains.deferred_init(4) == 4
  end

  test "returns propagate out of then and else" do
    assert LetChains.return_in_else(1) == -1
    assert LetChains.return_in_else(4) == 5
    assert LetChains.return_in_then(1) == 0
    assert LetChains.return_in_then(4) == 4
  end

  test "else runs once, whichever operand fails" do
    assert """
           in range 5
           """ == capture_io(fn -> LetChains.big_else(5) end)

    assert """
           out of range
           not between 2 and 9
           or missing
           """ == capture_io(fn -> LetChains.big_else(12) end)

    assert """
           out of range
           not between 2 and 9
           or missing
           """ == capture_io(fn -> LetChains.big_else(-1) end)
  end

  test "an else-if ladder of let chains picks the first matching chain" do
    assert LetChains.ladder(11, 0) == 1
    assert LetChains.ladder(0, 11) == 2
    assert LetChains.ladder(3, 3) == 3
    assert LetChains.ladder(-1, 3) == 4
  end

  test "while-let chains loop until an operand fails, with break and continue" do
    assert LetChains.while_let_chain(5) == 806
    assert LetChains.while_let_chain(20) == 1908
  end

  test "let chains work inside loop bodies" do
    assert LetChains.in_loop(5) == 302
  end

  test "let chains work inside closures" do
    assert LetChains.in_closure(-1) == 0
    assert LetChains.in_closure(1) == 2
    assert LetChains.in_closure(4) == 9
  end
end
