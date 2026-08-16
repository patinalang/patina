ExUnit.start()

defmodule Impl02Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main reads fields through concrete and generic impls" do
    assert """
           3, 42
           """ == capture_io(fn -> Patina.Impl02.main() end)
  end
end
