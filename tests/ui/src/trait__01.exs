ExUnit.start()

defmodule Trait01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  alias Patina.Trait01.Sheep

  @sheep_witness {:"impl Trait01::Animal for Trait01::Sheep", Sheep}

  defp dolly, do: Sheep."Trait01::Animal::new"(@sheep_witness, "Dolly")

  defp sheared_dolly do
    {sheared, _output} = with_io(fn -> Sheep.shear(dolly()) end)
    sheared
  end

  test "main talks, shears a copy and talks again" do
    assert """
           Dolly pauses briefly... baaaaah!
           Dolly gets a haircut!
           Dolly pauses briefly... baaaaah!
           """ == capture_io(fn -> Patina.Trait01.main() end)
  end

  test "Animal::new builds a Sheep native record" do
    assert [:naked, :name] = :records.get_field_names(dolly())
  end

  test "Animal::name returns the name" do
    assert "Dolly" = Sheep."Trait01::Animal::name"(@sheep_witness, dolly())
  end

  test "Animal::noise depends on whether the sheep is naked" do
    assert "baaaaah!" = Sheep."Trait01::Animal::noise"(@sheep_witness, dolly())
    assert "baaaaah?" = Sheep."Trait01::Animal::noise"(@sheep_witness, sheared_dolly())
  end

  test "shear gives a clothed sheep a haircut" do
    assert """
           Dolly gets a haircut!
           """ == capture_io(fn -> Sheep.shear(dolly()) end)
  end

  test "shear leaves a naked sheep as it is" do
    sheared = sheared_dolly()

    assert """
           Dolly is already naked...
           """ == capture_io(fn -> Sheep.shear(sheared) end)
  end

  test "the overridden Animal::talk pauses briefly" do
    assert """
           Dolly pauses briefly... baaaaah?
           """ ==
             capture_io(fn -> Sheep."Trait01::Animal::talk"(@sheep_witness, sheared_dolly()) end)
  end

  test "is_naked/1 is private" do
    refute function_exported?(Sheep, :is_naked, 1)
  end
end
