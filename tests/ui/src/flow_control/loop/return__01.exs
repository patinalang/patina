ExUnit.start()

defmodule Return01Test do
  use ExUnit.Case

  test "main's assert_eq! on the loop's break value passes" do
    assert {} == Patina.Return01.main()
  end
end
