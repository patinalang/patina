ExUnit.start()

defmodule TupleFieldsTest do
  use ExUnit.Case

  alias Patina.TupleFields

  test "first field" do
    assert 1 == TupleFields.first_of_three({1, "a", true})
  end

  test "last field" do
    assert true == TupleFields.last_of_three({1, "a", true})
  end

  test "field of a nested tuple" do
    assert 2.5 == TupleFields.nested({1, {"a", 2.5}})
  end

  test "field of a tuple with generic elements" do
    assert "a" == TupleFields.generic_second(1, "a")
  end

  test "field of a one-element tuple" do
    assert :x == TupleFields.single({:x})
  end

  test "field of a call result" do
    assert 2 == TupleFields.of_call(1)
  end
end
