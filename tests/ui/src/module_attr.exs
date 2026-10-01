ExUnit.start()

defmodule ModuleAttrTest do
  use ExUnit.Case

  test "module overrides apply to transparent struct modules" do
    assert ModuleAttr.Person.__info__(:struct) == nil
  end

  test "module overrides apply to enums" do
    Code.ensure_loaded!(:module_attr_web_event)

    assert {%{is_exported: true}, [:x, :y]} =
             :records.get_definition(:module_attr_web_event, :Click)
  end

  test "module overrides apply to modules" do
    assert :module_attr_helpers.answer() == 42
  end
end
