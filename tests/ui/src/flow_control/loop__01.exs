ExUnit.start()

defmodule Loop01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main counts until break, skipping the continued iteration" do
    assert """
           Let's count until infinity!
           1
           2
           three
           4
           5
           OK, that's enough
           """ == capture_io(fn -> Patina.Loop01.main() end)
  end
end
