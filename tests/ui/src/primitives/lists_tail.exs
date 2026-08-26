ExUnit.start()

defmodule ListsTailTest do
  use ExUnit.Case

  alias Patina.ListsTail

  test "prepends elements onto an existing list tail" do
    assert ListsTail.prepend([1, 2, 3]) == [-1, 0, 1, 2, 3]
  end

  test "the explicit tail round-trips through match and let-else" do
    xs = [1, 2, 3]
    assert ListsTail.match_tail(xs) == xs
    assert ListsTail.let_else_tail(xs) == xs
  end
end
