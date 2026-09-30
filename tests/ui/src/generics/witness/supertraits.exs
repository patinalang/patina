ExUnit.start()

defmodule SupertraitsTest do
  use ExUnit.Case

  alias Patina.Supertraits

  test "a generic function calls the methods of the supertraits of its bound" do
    assert {"Bob", {"Hi", "Bob"}} == Supertraits.bob_intro()
    assert {"Robot", {"Beep", "Robot"}} == Supertraits.robot_intro()
  end

  test "an indirect supertrait comes through the direct supertrait" do
    assert {"Bob", {"Bob", {"Hi", "Bob"}}} == Supertraits.bob_intro_loud()
  end

  test "a default body gets a supertrait witness and an own bound witness" do
    assert {4, true} == Supertraits.books_count(4)
    assert {3, false} == Supertraits.books_count(3)
  end
end
