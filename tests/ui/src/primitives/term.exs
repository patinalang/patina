ExUnit.start()

defmodule TermTest do
  use ExUnit.Case

  alias Patina.Term

  test "== is true for equal terms" do
    assert Term.equal({:a, [1, %{2 => "b"}]}, {:a, [1, %{2 => "b"}]})
  end

  test "== is false for different terms" do
    refute Term.equal({:a, 1}, {:a, 2})
  end

  test "== compares exactly, so an integer and an equal float differ" do
    refute Term.equal(1, 1.0)
  end

  test "!= is true for different terms" do
    assert Term.not_equal(:a, :b)
  end

  test "!= is false for equal terms" do
    refute Term.not_equal(:a, :a)
  end

  test "PartialEq for term compares exactly" do
    assert Term.partial_eq_equal([1, 2], [1, 2])
    refute Term.partial_eq_equal(1, 1.0)
  end
end
