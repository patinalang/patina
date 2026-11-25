ExUnit.start()

defmodule EscapedKeywordsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "formats captured and explicit keyword arguments" do
    assert capture_io(&Patina.EscapedKeywords.print/0) == "1\n2\n3\n"
  end
end
