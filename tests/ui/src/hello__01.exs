ExUnit.start()

defmodule Hello01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main prints Hello World!" do
    assert """
           Hello World!
           """ == capture_io(fn -> Patina.Hello01.main() end)
  end

  test "has Elixir __info__ function" do
    assert Patina.Hello01.__info__(:functions) == [{:main, 0}]
    assert Patina.Hello01.__info__(:macros) == []
    assert Patina.Hello01.__info__(:struct) == nil

    assert Patina.Hello01.__info__(:exports_md5) ==
             :erlang.md5(
               :erlang.term_to_binary(
                 {:lists.sort(Patina.Hello01.module_info(:exports)), [], nil},
                 [
                   :deterministic
                 ]
               )
             )

    assert Patina.Hello01.__info__(:deprecated) == []
    assert Patina.Hello01.__info__(:attributes) == Patina.Hello01.module_info(:attributes)
    assert Patina.Hello01.__info__(:compile) == Patina.Hello01.module_info(:compile)
    assert Patina.Hello01.__info__(:md5) == Patina.Hello01.module_info(:md5)
  end
end
