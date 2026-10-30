ExUnit.start()

defmodule ForeignImplTest do
  use ExUnit.Case

  alias Patina.ForeignImpl

  test "a local trait on a foreign type" do
    assert 5 == ForeignImpl.label_size()
  end

  test "a foreign trait with a local argument, on a foreign type" do
    assert 7 == ForeignImpl.local_to_celsius(7)
  end

  test "impls of one trait for one type from the type's package and from another package" do
    assert 100 == ForeignImpl.fahrenheit_to_celsius(212)
    assert 27 == ForeignImpl.kelvin_to_celsius(300)
    assert -3 == ForeignImpl.rankine_to_celsius(486)
  end

  test "two independent packages implement different traits for one foreign type" do
    assert "SHOUT" == ForeignImpl.label_shout()
    assert "whisper" == ForeignImpl.label_whisper()
    assert "shown" == ForeignImpl.label_show()
  end

  test "a blanket impl of another package" do
    assert "hi!" == ForeignImpl.blanket_on_foreign_type("hi")
    assert "local!" == ForeignImpl.blanket_on_local_type()
    assert "hi!local!" == ForeignImpl.blanket_generic_both("hi")
  end
end
