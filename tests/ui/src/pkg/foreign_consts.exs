ExUnit.start()

defmodule PkgForeignConstsTest do
  use ExUnit.Case

  alias Patina.ForeignConsts

  test "named constants of another package are inlined" do
    pair = {[1, 2, 3], %{"answer" => 42}}
    assert pair == ForeignConsts.pair()
    assert [pair, pair] == ForeignConsts.local_pairs()
    assert <<"PT", 2::16>> == ForeignConsts.header()
    assert 0 == ForeignConsts.origin_y()
    assert 1 == ForeignConsts.computed_unit_x()
    assert 6 == ForeignConsts.foreign_doubled_of_local_impl()
    assert [] == ForeignConsts.generic_empty()
  end

  test "named constants of another package can be patterns" do
    assert ForeignConsts.is_pair({[1, 2, 3], %{"answer" => 42}})
    refute ForeignConsts.is_pair({[1, 2, 3], %{"answer" => 42, "extra" => 0}})
    assert ForeignConsts.is_header(<<"PT", 2::16>>)
    refute ForeignConsts.is_header(<<"PT", 3::16>>)
  end
end
