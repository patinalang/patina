ExUnit.start()

defmodule MultiImplTest do
  use ExUnit.Case

  alias Patina.MultiImpl
  alias Patina.MultiImpl.Celsius

  test "a generic call selects the impl through the witness" do
    assert 100 == MultiImpl.from_fahrenheit(212)
    assert 27 == MultiImpl.from_kelvin(300)
  end

  test "a static call selects the impl through the witness" do
    assert 27 == MultiImpl.from_kelvin_static(300)
  end

  test "each impl has its own function" do
    fahrenheit = Celsius."MultiImpl::From<MultiImpl::Fahrenheit>::$patina_witness"()
    kelvin = Celsius."MultiImpl::From<MultiImpl::Kelvin>::$patina_witness"()

    assert 100 == Celsius."MultiImpl::From<MultiImpl::Fahrenheit>::from"(fahrenheit, 212)
    assert -61 == Celsius."MultiImpl::From<MultiImpl::Kelvin>::from"(kelvin, 212)

    {{from}, {}, {}} = fahrenheit
    assert 100 == from.(fahrenheit, 212)
    {{from}, {}, {}} = kelvin
    assert -61 == from.(kelvin, 212)
  end
end
