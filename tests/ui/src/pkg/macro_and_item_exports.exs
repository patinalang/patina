ExUnit.start()

defmodule MacroAndItemExportsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "a package may export a trait and its derive alongside ordinary items" do
    assert """
           5 7 42
           """ == capture_io(fn -> Patina.MacroAndItemExports.items() end)
  end

  test "exported macro_rules! and proc macros coexist in one package" do
    assert """
           10 7
           """ == capture_io(fn -> Patina.MacroAndItemExports.macros() end)
  end
end
