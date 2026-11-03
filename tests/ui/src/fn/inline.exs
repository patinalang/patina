ExUnit.start()

defmodule InlineTest do
  use ExUnit.Case

  alias Patina.Inline

  test "inlined calls bind their arguments to the callee's params" do
    assert Inline.inlines_free_fn() == 3
    assert Inline.binds_pattern_param() == {2, 1}
    assert Inline.inlines_nested_inline_calls() == 5
    assert Inline.inlines_each_call_separately() == 21
  end

  test "inlined bodies keep their control flow" do
    assert Inline.return_leaves_only_the_copy(3) == 4
    assert Inline.return_leaves_only_the_copy(20) == 11
    assert Inline.inlines_loop() == 6
    assert Inline.inlines_closure() == 3
    assert Inline.recursion_inlines_once() == 3
  end

  test "inline functions can be used as values" do
    assert Inline.fn_item_value_stays_reference().(5, 6) == 11
    assert Inline.call_through_fn_item_inlines() == 11
  end

  test "methods inline where they resolve statically" do
    assert Inline.inlines_inherent_method() == 3
    assert Inline.inlines_impl_method() == 9
    assert Inline.inlines_default_method() == 18
    assert Inline.copy_calls_impl_method_statically() == 6
    assert Inline.copy_inlines_resolved_trait_method() == 16
    assert Inline.generic_caller_dispatches_through_witness() == 25
    assert Inline.copy_builds_witness_for_generic_call() == 10
  end

  test "constants of inlined copies are bound once" do
    assert Inline.binds_const_once_for_all_copies() == 18
  end

  test "functions called from inline functions are exported" do
    assert Inline.exports_helpers_of_inline_fns() == 40
    assert {:secret, 1} in Inline.Helpers.module_info(:exports)
  end
end
