ExUnit.start()

defmodule PidPortRefTest do
  use ExUnit.Case

  alias Patina.PidPortRef

  describe "equality" do
    test "pid" do
      [left, right] = pids()

      assert PidPortRef.pids_equal(left, left)
      refute PidPortRef.pids_equal(left, right)
      assert PidPortRef.pids_equal_generic(left, left)
      refute PidPortRef.pids_equal_generic(left, right)
    end

    test "port" do
      [left, right] = ports()

      assert PidPortRef.ports_equal(left, left)
      refute PidPortRef.ports_equal(left, right)
      assert PidPortRef.ports_equal_generic(left, left)
      refute PidPortRef.ports_equal_generic(left, right)
    end

    test "ref" do
      [left, right] = refs()

      assert PidPortRef.refs_equal(left, left)
      refute PidPortRef.refs_equal(left, right)
      assert PidPortRef.refs_equal_generic(left, left)
      refute PidPortRef.refs_equal_generic(left, right)
    end
  end

  describe "ordering" do
    test "pid" do
      [low, high] = pids()

      assert PidPortRef.pids_less_than(low, high)
      refute PidPortRef.pids_less_than(high, low)
      assert PidPortRef.pids_less_than_generic(low, high)
      refute PidPortRef.pids_less_than_generic(high, low)
    end

    test "port" do
      [low, high] = ports()

      assert PidPortRef.ports_less_than(low, high)
      refute PidPortRef.ports_less_than(high, low)
      assert PidPortRef.ports_less_than_generic(low, high)
      refute PidPortRef.ports_less_than_generic(high, low)
    end

    test "ref" do
      [low, high] = refs()

      assert PidPortRef.refs_less_than(low, high)
      refute PidPortRef.refs_less_than(high, low)
      assert PidPortRef.refs_less_than_generic(low, high)
      refute PidPortRef.refs_less_than_generic(high, low)
    end
  end

  describe "class patterns" do
    test "pid" do
      assert PidPortRef.kind(self()) == :pid
    end

    test "port" do
      assert PidPortRef.kind(open_port()) == :port
    end

    test "ref" do
      assert PidPortRef.kind(make_ref()) == :ref
    end

    test "other" do
      assert PidPortRef.kind(:other) == :other
    end
  end

  defp pids, do: Enum.sort([self(), spawn(fn -> :ok end)])
  defp ports, do: Enum.sort([open_port(), open_port()])
  defp refs, do: Enum.sort([make_ref(), make_ref()])
  defp open_port, do: Port.open({:spawn, "cat"}, [])
end
