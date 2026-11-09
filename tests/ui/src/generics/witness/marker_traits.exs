ExUnit.start()

defmodule MarkerTraitsTest do
  use ExUnit.Case

  alias Patina.MarkerTraits
  alias Patina.MarkerTraits.A

  test "a bound on a trait without items takes no witness" do
    assert "a" == MarkerTraits.a_marked()
    assert :anything == MarkerTraits.marker_bounds_only(:anything)
  end

  test "a trait without items gives the witnesses of its supertraits" do
    assert "a" == MarkerTraits.a_both()

    # The bounds tuple holds only the witness of `A: Show`; `A: Marker` has none.
    show_a = A."marker_traits::Show::$patina_witness"()
    assert "a" == MarkerTraits.subtrait_of_marker({show_a}, A)
  end
end
