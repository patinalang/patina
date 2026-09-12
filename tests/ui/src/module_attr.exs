ExUnit.start()

defmodule ModuleAttrTest do
  use ExUnit.Case

  setup_all do
    Code.ensure_all_loaded!([ModuleAttr.Person, :module_attr_web_event, :module_attr_helpers])
  end

  test "module overrides apply to transparent struct modules" do
    assert ModuleAttr.Person.__info__(:struct) == nil
  end

  test "module overrides apply to enums" do
    assert {%{is_exported: true}, [:x, :y]} =
             :records.get_definition(:module_attr_web_event, :Click)
  end

  test "module overrides apply to modules" do
    assert :module_attr_helpers.answer() == 42
  end
end
