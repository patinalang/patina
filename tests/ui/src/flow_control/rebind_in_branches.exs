ExUnit.start()

defmodule RebindInBranchesTest do
  use ExUnit.Case

  alias Patina.RebindInBranches

  test "each branch rebinds from the same incoming value" do
    assert RebindInBranches.branch_result(true) == 111
    assert RebindInBranches.branch_result(false) == 212
  end

  test "the last write in every branch shares the post-join version" do
    assert RebindInBranches.latest_version_wins(true) == 3
    assert RebindInBranches.latest_version_wins(false) == 4
  end

  test "each branch can initialize the same outer binding" do
    assert RebindInBranches.initialize_in_branches(true) == 10
    assert RebindInBranches.initialize_in_branches(false) == 20
  end

  test "a diverging branch does not need to initialize an outer binding" do
    assert RebindInBranches.initialize_or_return(true) == 30
    assert RebindInBranches.initialize_or_return(false) == 40
  end

  test "modified bindings are threaded through nested blocks" do
    assert RebindInBranches.nested_result() == 1113
  end

  test "inner joins feed their reaching bindings to outer joins" do
    assert RebindInBranches.nested_branches(true, true) == 111
    assert RebindInBranches.nested_branches(true, false) == 122
    assert RebindInBranches.nested_branches(false, false) == 303
  end

  test "short-circuited blocks forward their incoming bindings" do
    assert RebindInBranches.short_circuit_result(true) == 15
    assert RebindInBranches.short_circuit_result(false) == 10
  end

  test "match arms rebind from the same incoming value" do
    assert RebindInBranches.match_result(true) == 111
    assert RebindInBranches.match_result(false) == 212
  end
end
