ExUnit.start()

defmodule LocalExportTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "private trait methods called across generated modules are exported" do
    assert ~S"""
           Priv::p('Elixir.Patina.LocalExport.S')
           """ == capture_io(fn -> Patina.LocalExport.main() end)
  end
end
