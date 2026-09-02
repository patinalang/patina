ExUnit.start()

defmodule Enum01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Enum01.WebEvent

  setup_all do
    Code.ensure_all_loaded!([WebEvent])
  end

  test "main destructures every enum variant" do
    assert """
           pressed '120'.
           pasted "my text".
           clicked at x=20, y=80.
           page loaded
           page unloaded
           """ == capture_io(fn -> Patina.Enum01.main() end)
  end

  test "enum module exports its variant record definitions" do
    Enum.each(
      [
        {:KeyPress, [:"0"]},
        {:Paste, [:"0"]},
        {:Click, [:x, :y]}
      ],
      fn {variant, fields} ->
        assert {%{is_exported: true}, ^fields} = :records.get_definition(WebEvent, variant)
      end
    )
  end
end
