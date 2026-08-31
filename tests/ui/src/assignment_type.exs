ExUnit.start()

defmodule AssignmentTypeTest do
  use ExUnit.Case

  test "assignment expressions evaluate to the assigned value" do
    assert 42 == Patina.AssignmentType.assign()
    assert 42 == Patina.AssignmentType.assign_op()
    assert 21 == Patina.AssignmentType.chained_assign()
    assert 42 == Patina.AssignmentType.assign_as_argument()
    assert 42 == Patina.AssignmentType.assign_as_match_scrutinee()
    assert true == Patina.AssignmentType.assign_in_boolean_context()
  end

  test "blocks without tail expressions evaluate to unit" do
    assert {} == Patina.AssignmentType.empty_block()
    assert {} == Patina.AssignmentType.block_ending_in_let()
    assert {} == Patina.AssignmentType.block_ending_in_uninitialized_let()
    assert {} == Patina.AssignmentType.block_ending_in_let_else(42)
    assert {} == Patina.AssignmentType.block_ending_in_let_else(0)
    assert {} == Patina.AssignmentType.block_ending_in_unit_expr()
    assert {} == Patina.AssignmentType.block_ending_in_assignment()
  end
end
