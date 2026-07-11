ExUnit.start()

defmodule DestructureEnum01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.DestructureEnum01.Color

  setup_all do
    Code.ensure_all_loaded!([Color])
  end

  test "main destructures the RGB enum variant" do
    assert """
           What color is it?
           Red: 122, green: 17, and blue: 40!
           """ == capture_io(fn -> Patina.DestructureEnum01.main() end)
  end

  test "enum module exports its variant record definitions" do
    Enum.each(
      [
        {:Red, []},
        {:Blue, []},
        {:Green, []},
        {:RGB, [:"0", :"1", :"2"]},
        {:HSV, [:"0", :"1", :"2"]},
        {:HSL, [:"0", :"1", :"2"]},
        {:CMY, [:"0", :"1", :"2"]},
        {:CMYK, [:"0", :"1", :"2", :"3"]}
      ],
      fn {variant, fields} ->
        assert {%{is_exported: true}, ^fields} = :records.get_definition(Color, variant)
      end
    )
  end
end
