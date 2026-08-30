ExUnit.start()

defmodule BacktickIdent01Test do
  use ExUnit.Case

  alias Patina.BacktickIdent

  test "backtick-named functions are exported and callable" do
    assert BacktickIdent.exists?(1)
    BacktickIdent.save!()
    assert apply(BacktickIdent, :"a{b}", []) == 1
    assert BacktickIdent.type() == 2
  end
end
