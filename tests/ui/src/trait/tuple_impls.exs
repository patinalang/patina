ExUnit.start()

defmodule TupleImplsTest do
  use ExUnit.Case

  alias Patina.TupleImpls

  test "impls on tuples of each arity" do
    assert "()" == TupleImpls.show_unit()
    assert "(i,x,)" == TupleImpls.show_pair(1, "x")
    assert "(i,)(i,x,)(i,x,(i,(),),)" == TupleImpls.show_generic_all(1, "x")
  end

  test "impls on tuples are hosted in the tuple host module" do
    assert {:module, _} = Code.ensure_loaded(:"Elixir.Patina.TupleImpls.$impl.Core.Tuple")
  end
end
