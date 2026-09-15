ExUnit.start()

defmodule Receive01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the selected receive arms and timeout" do
    send(self(), {:unmatched, 1})
    send(self(), 42)
    send(self(), :ping)
    send(self(), 7)
    send(self(), :hello)
    send(self(), 9)
    send(self(), {:unmatched, 2})

    assert """
           received integer 42
           received any message {unmatched,1}
           received ping
           received any message 7
           received atom hello
           received any message 9
           timeout (5 seconds with no matching message)
           received any message {unmatched,2}
           """ ==
             capture_io(fn ->
               Patina.Receive01.main()
               Patina.Receive01.main()
               Patina.Receive01.main()
               Patina.Receive01.main()
             end)
  end
end
