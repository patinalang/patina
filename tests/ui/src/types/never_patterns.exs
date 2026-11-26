ExUnit.start()

defmodule NeverPatternsTest do
  use ExUnit.Case

  alias Patina.NeverPatterns

  test "an impossible arm is dropped wherever it is" do
    assert 1 == NeverPatterns.option_none_first()
    assert 1 == NeverPatterns.option_some_first()
  end

  test "an impossible or-pattern alternative is dropped" do
    assert 5 == NeverPatterns.or_pattern_b(5)
    assert 0 == NeverPatterns.or_pattern_c()
  end

  test "an arm made impossible by inlining is dropped" do
    assert false == NeverPatterns.inlined_none()
    assert true == NeverPatterns.generic_some(3)
  end

  test "constructing a variant with a never payload diverges" do
    assert_raise Patina.Core.Panic, fn -> NeverPatterns.construct_some() end
  end

  test "a match with no possible arms has no clauses" do
    assert_raise CaseClauseError, fn -> NeverPatterns.no_possible_arms(:anything) end
  end
end
