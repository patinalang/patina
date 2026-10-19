ExUnit.start()

defmodule DuplicatePreconditionsTest do
  use ExUnit.Case

  alias Patina.DuplicatePreconditions

  test "an impl whose where-clauses coincide is still usable" do
    assert 33 == DuplicatePreconditions.duplicate_where_clauses()
  end
end
