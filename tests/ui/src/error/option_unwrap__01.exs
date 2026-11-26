ExUnit.start()

defmodule OptionUnwrap01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints the drink responses and panics when unwrapping None" do
    assert """
           water? How nice.
           Yuck! Too sugary.
           No drink? Oh well.
           I love coffees!!!!!
           """ ==
             capture_io(fn ->
               assert_raise Patina.Core.Panic,
                            "called `Option::unwrap()` on a `None` value",
                            &Patina.OptionUnwrap01.main/0
             end)
  end
end
