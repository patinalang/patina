ExUnit.start()

defmodule AtomTest do
  use ExUnit.Case

  alias Patina.Atom

  test "atom literals survive compilation and ETF encoding" do
    assert Atom.annotated() == :hello
    assert Atom.named() == :named
    assert Atom.escaped() == :"line\nbreak"
    assert Atom.raw() == :"no \\escapes"
    assert Atom.unicode() == :"zażółć"
  end

  test "atoms support comparison and literal patterns" do
    assert Atom.equal(:same, :same)
    refute Atom.equal(:same, :different)
    assert Atom.less_than(:alpha, :omega)
    assert Atom.classify(:hello) == :greeting
    assert Atom.classify(:goodbye) == :farewell
    assert Atom.classify(:other) == :unknown
  end
end
