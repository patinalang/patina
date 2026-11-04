ExUnit.start()

defmodule Disambiguating01Test do
  use ExUnit.Case

  alias Patina.Disambiguating01.Form

  @username_witness Form."Disambiguating01::UsernameWidget::$patina_witness"()
  @age_witness Form."Disambiguating01::AgeWidget::$patina_witness"()

  defp form, do: Form."$patina_constructor__Form"("rustacean", 28)

  test "main's assert_eq!s pass" do
    assert {} == Patina.Disambiguating01.main()
  end

  test "UsernameWidget::get returns the username" do
    assert "rustacean" == Form."Disambiguating01::UsernameWidget::get"(@username_witness, form())
  end

  test "AgeWidget::get returns the age" do
    assert 28 == Form."Disambiguating01::AgeWidget::get"(@age_witness, form())
  end
end
