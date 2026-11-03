ExUnit.start()

defmodule PkgForeignInlineTest do
  use ExUnit.Case

  alias Patina.ForeignInline

  test "inline functions of another package are inlined" do
    assert 10 == ForeignInline.copy_calls_private_fn()
    assert 3 == ForeignInline.inlines_foreign_impl_method()
    assert 6 == ForeignInline.inlines_foreign_generic_with_foreign_impl()
    assert 10 == ForeignInline.foreign_generic_calls_local_impl_statically()
  end
end
