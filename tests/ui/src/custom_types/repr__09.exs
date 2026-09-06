ExUnit.start()

defmodule Repr09Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "open-shape construction wraps exactly the ambiguous values" do
    assert """
           {othertag,<<"hello">>}
           42
           hi
           {othertag,{othertag,1}}
           """ == capture_io(fn -> Patina.Repr09.main() end)
  end
end
