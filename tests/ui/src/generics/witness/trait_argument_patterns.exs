ExUnit.start()

defmodule TraitArgumentPatternsTest do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "bound and unbound impl trait arguments accept concrete witnesses" do
    assert ~S"""
           UnboundContainer::contains('Elixir.Patina.TraitArgumentPatterns.UnboundContainer', 1, false)
           true
           BoundContainer::contains('Elixir.Patina.TraitArgumentPatterns.BoundContainer', 1, #{'__struct__' => 'Elixir.Patina.TraitArgumentPatterns.Marked',value => 2})
           Marker::value(#{'__struct__' => 'Elixir.Patina.TraitArgumentPatterns.Marked',value => 2})
           true
           NestedContainer::contains('Elixir.Patina.TraitArgumentPatterns.NestedContainer', 1, #{'__struct__' => 'Elixir.Patina.TraitArgumentPatterns.Wrapped',
             value => false})
           true
           """ == capture_io(fn -> Patina.TraitArgumentPatterns.main() end)
  end
end
