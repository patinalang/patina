ExUnit.start()

defmodule LetElseTest do
  use ExUnit.Case

  alias Patina.LetElse

  test "let-else binds a successful tuple pattern" do
    assert LetElse.tuple_pattern() == 42
  end

  test "let-else binds a matching list and takes else for a short one" do
    assert LetElse.list_pattern([10, 20]) == 30
    assert LetElse.list_pattern([10]) == 0
  end
end
