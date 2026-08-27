ExUnit.start()

defmodule IfLet03Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.IfLet03

  setup_all do
    Code.ensure_all_loaded!([IfLet03.Foo])
  end

  test "main prints the matching if-let branches" do
    assert """
           a is foobar
           c is 100
           c is one hundred
           """ == capture_io(fn -> IfLet03.main() end)
  end
end
