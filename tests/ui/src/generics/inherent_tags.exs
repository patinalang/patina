ExUnit.start()

defmodule InherentTagsTest do
  use ExUnit.Case

  alias Patina.InherentTags
  alias Patina.InherentTags.GenVal
  alias Patina.InherentTags.Name

  test "methods of the same name on different arguments are different functions" do
    assert 5 == InherentTags.int_value(5)
    assert "hi" == InherentTags.str_value("hi")
    assert 5 == GenVal."value<int>"(5)
    assert "hi" == GenVal."value<str>"("hi")
  end

  test "a fully generic impl keeps plain names" do
    assert "a name" == InherentTags.name_f()
    assert 7 == InherentTags.int_f(5, 2)
    assert 7 == GenVal."f<int>"(5, 2)

    # The bounds tuple holds the witness of `Name: Show`: the impl's tag, then its host module.
    show_name = {:"impl InherentTags::Show for InherentTags::Name", Name}
    assert "a name" == GenVal.f({show_name}, Name)
  end
end
