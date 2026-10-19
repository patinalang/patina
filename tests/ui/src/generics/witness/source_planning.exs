ExUnit.start()

defmodule SourcePlanningTest do
  use ExUnit.Case

  alias Patina.SourcePlanning

  test "a later source improves earlier uses" do
    assert {10, 20, 10} == SourcePlanning.late_source()
    assert {10, 20, 10} == SourcePlanning.early_source()
  end

  test "a constructor input is available before construction" do
    assert {30, 30} == SourcePlanning.constructor_input()
  end

  test "loop captures use the final source" do
    assert 20 == SourcePlanning.loop_source(0)
    assert 50 == SourcePlanning.loop_source(3)
  end

  test "a direct associated bound beats an indirect path" do
    assert 20 == SourcePlanning.associated_source()
  end
end
