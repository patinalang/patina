ExUnit.start()

defmodule TraitDefaultsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "default bodies are shared across the package boundary" do
    assert """
           Hello, Ann!
           Hi, cat!
           Hi, cat!
           Hello, Bob!
           Hi, cat!
           Hi, robot!
           """ == capture_io(fn -> Patina.TraitDefaults.main() end)
  end
end
