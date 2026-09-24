ExUnit.start()

defmodule StrUtf8Test do
  use ExUnit.Case

  alias Patina.StrUtf8

  test "non-ASCII str literals are UTF-8 encoded" do
    assert StrUtf8.greeting() == "héllo, wörld ∞"
    assert byte_size(StrUtf8.greeting()) == 18
  end

  test "a str literal expression matches the same str literal pattern" do
    assert StrUtf8.is_greeting(StrUtf8.greeting())
    assert StrUtf8.is_greeting("héllo, wörld ∞")
    refute StrUtf8.is_greeting("hello, world")
  end
end
