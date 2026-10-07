ExUnit.start()

defmodule For03Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main greets every name and still has the list afterwards" do
    assert """
           Hello Bob
           Hello Frank
           There is a rustacean among us!
           names: [<<"Bob">>,<<"Frank">>,<<"Ferris">>]
           """ == capture_io(fn -> Patina.For03.main() end)
  end
end
