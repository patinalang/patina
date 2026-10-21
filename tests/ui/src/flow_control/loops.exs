ExUnit.start()

defmodule LoopsTest do
  use ExUnit.Case

  alias Patina.Loops

  setup do
    Process.put(:notes, [])
    :ok
  end

  test "generated loop helpers are private in the final BEAM" do
    refute Enum.any?(Loops.module_info(:exports), fn {name, _arity} ->
             String.contains?(Atom.to_string(name), "$patina_loop_")
           end)
  end

  test "normal fallthrough recurses and continue skips the rest of an iteration" do
    assert Loops.normal_continue(5) == 905
  end

  test "loop values and rebound variables reach the caller" do
    assert Loops.loop_value(4) == 84
  end

  test "loops support no parameters and initialize outer bindings" do
    assert Loops.zero_parameter_loop() == 7
    assert Loops.initialize_in_loop() == 7
  end

  test "while threads state, including its zero-iteration path" do
    assert Loops.while_sum(1, 3) == 303
    assert Loops.while_sum(5, 5) == 5
  end

  test "nested loops keep independent break and continue targets" do
    assert Loops.nested(2, 3) == 3602
  end

  test "return propagates through generated loop helpers" do
    assert Loops.return_from_loop(4) == 40
    assert Loops.return_from_loop(20) == -1
    assert Loops.return_from_nested(2, 2) == 202
    assert Loops.return_from_nested(4, 4) == -1
    assert Loops.return_from_nested_uneven_rebinds(2, 2) == 2202
    assert Loops.return_from_nested_uneven_rebinds(4, 4) == -1
  end

  test "labeled continue returns a continue payload instead of recursing the current helper" do
    assert Loops.continue_outer(3) == 303
    assert Loops.continue_through_middle(3) == 303
  end

  test "a cast around a loop preserves the loop's break target" do
    assert Loops.cast_loop_value() == 1
  end

  defp loop_helpers(module, fun_name) do
    for {name, _arity} <- module.module_info(:functions),
        name = Atom.to_string(name),
        String.starts_with?(name, fun_name) and String.contains?(name, "$patina_loop_"),
        into: MapSet.new(),
        do: name
  end

  test "loop helpers are numbered from the top of the function, including nested and closure loops" do
    assert loop_helpers(Loops, "helper_names") ==
             MapSet.new(for n <- 1..5, do: "helper_names$patina_loop_#{n}")
  end

  test "loop helpers in trait impls are named after the impl's function" do
    assert loop_helpers(Loops.Counter, "Loops::Count::count") ==
             MapSet.new([
               "Loops::Count::count<int>$patina_loop_1",
               "Loops::Count::count<str>$patina_loop_1"
             ])
  end

  test "continue and break in a try body run the after body" do
    assert Loops.continue_and_break_in_try(3) == 30
    assert Process.put(:notes, []) == [3, 2, 1]

    assert Loops.continue_and_break_in_try(1) == 10
    assert Process.put(:notes, []) == [1]
  end

  test "break in a catch arm runs the after body" do
    assert Loops.break_in_catch(5, fn n -> if n == 3, do: :erlang.error(:boom), else: :ok end) ==
             3

    assert Process.put(:notes, []) == [3, 2, 1]

    assert Loops.break_in_catch(2, fn n -> if n == 3, do: :erlang.error(:boom), else: :ok end) ==
             -1

    assert Process.put(:notes, []) == [2, 1]
  end

  test "continue in a catch arm runs the after body" do
    assert Loops.continue_in_catch(4, fn n ->
             if rem(n, 2) == 0, do: :erlang.error(:boom), else: :ok
           end) == 2

    assert Process.put(:notes, []) == [4, 3, 2, 1]
  end

  test "break and continue in an after body" do
    assert Loops.break_and_continue_in_after(5) == 52
    assert Process.put(:notes, []) == [5, 4, 3, 2, 1]
  end

  test "return in a try body inside a loop runs the after body" do
    assert Loops.return_in_try_in_loop(3) == 30
    assert Process.put(:notes, []) == [3, 2, 1]

    assert Loops.return_in_try_in_loop(9) == -1
    assert Process.put(:notes, []) == [5, 4, 3, 2, 1]
  end

  test "return in an after body inside a loop" do
    assert Loops.return_in_after_in_loop(3) == 30
    assert Process.put(:notes, []) == [3, 2, 1]

    assert Loops.return_in_after_in_loop(9) == -1
    assert Process.put(:notes, []) == [5, 4, 3, 2, 1]
  end

  test "labeled continue and break out of a try body run the after body" do
    assert Loops.continue_outer_from_try(3) == 303
    assert Process.put(:notes, []) == [3, 2, 1]
  end

  test "return and break from a loop inside a try body" do
    assert Loops.loop_in_try(3) == 3
    assert Process.put(:notes, []) == [0]

    assert Loops.loop_in_try(9) == 51
    assert Process.put(:notes, []) == [0]
  end
end
