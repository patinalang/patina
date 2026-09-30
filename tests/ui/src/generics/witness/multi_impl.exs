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

  test "the tag selects the clause of the impl" do
    assert 100 == Celsius."MultiImpl::From::from"({:"impl MultiImpl::From<MultiImpl::Fahrenheit> for MultiImpl::Celsius", Celsius}, 212)
    assert -61 == Celsius."MultiImpl::From::from"({:"impl MultiImpl::From<MultiImpl::Kelvin> for MultiImpl::Celsius", Celsius}, 212)
  end

  test "a tag that no impl has fails at the call" do
    assert_raise FunctionClauseError, fn ->
      Celsius."MultiImpl::From::from"({:"impl MultiImpl::From<MultiImpl::Rankine> for MultiImpl::Celsius", Celsius}, 212)
    end
  end
end
