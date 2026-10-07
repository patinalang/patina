ExUnit.start()

defmodule For01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints fizzbuzz for a half-open range" do
    expected =
      Enum.map_join(1..100, "\n", fn n ->
        case {rem(n, 3), rem(n, 5)} do
          {0, 0} -> "fizzbuzz"
          {0, _} -> "fizz"
          {_, 0} -> "buzz"
          {_, _} -> "#{n}"
        end
      end)

    assert expected <> "\n" == capture_io(fn -> Patina.For01.main() end)
  end
end
