ExUnit.start()

defmodule BacktickIdent01Test do
  use ExUnit.Case

  alias Patina.BacktickIdent

  test "backtick-named functions are exported and callable" do
    assert BacktickIdent.exists?(1)
    BacktickIdent.save!()
    assert apply(BacktickIdent, :"a{b}", []) == 1
    assert BacktickIdent.type() == 2
    assert BacktickIdent.keyword_module() == 42
    assert BacktickIdent.out_of_line_keyword_module() == 37
    assert BacktickIdent.keyword_trait() == 43
    assert function_exported?(Patina.BacktickIdent.S, :"backtick_ident::type::Shape::fn", 2)
    assert BacktickIdent.keyword_extern()
  end

  test "keyword names in nested functions and locals" do
    assert BacktickIdent.keyword_nested_fns() == 3
    assert BacktickIdent.keyword_local() == 5
  end

  test "literals with an escaped keyword's text stay unchanged" do
    assert BacktickIdent.keyword_literals() == {"`fn`", :"`fn`"}
  end

  test "block-local keyword traits and modules" do
    assert BacktickIdent.keyword_nested_traits() == 3
    assert BacktickIdent.keyword_nested_mods() == 3
  end

  test "Self is still a unit struct pattern" do
    assert BacktickIdent.unit_pattern()
  end
end
