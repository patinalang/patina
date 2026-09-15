ExUnit.start()

defmodule UnsafeReceive01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the selected receive arms and timeout" do
    send(self(), 42)
    send(self(), :hello)
    send(self(), nil)
    send(self(), {:some, "value"})
    send(self(), 7)

    assert """
           got integer 42
           got atom hello
           got atom nil
           got something else
           got integer 7
           timeout
           """ ==
             capture_io(fn ->
               Patina.UnsafeReceive01.main()
               Patina.UnsafeReceive01.main()
               Patina.UnsafeReceive01.main()
               Patina.UnsafeReceive01.main()
               Patina.UnsafeReceive01.main()
               Patina.UnsafeReceive01.main()
             end)
  end
end
