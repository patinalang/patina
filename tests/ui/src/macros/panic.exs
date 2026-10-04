ExUnit.start()

defmodule PanicTest do
  use ExUnit.Case

  defp format_panic(fun) do
    try do
      fun.()
      flunk("expected a panic")
    rescue
      e in Patina.Core.Panic -> Exception.format(:error, e, __STACKTRACE__)
    end
  end

  test "panic! raises a Patina.Core.Panic exception from the panicking function" do
    formatted = format_panic(&Patina.Panic.explicit_panic/0)
    assert formatted =~ "** (Patina.Core.Panic) explicit panic\n"
    assert formatted =~ "panic.pt:2: Patina.Panic.explicit_panic/0\n"

    formatted = format_panic(&Patina.Panic.panic_with_message/0)
    assert formatted =~ "** (Patina.Core.Panic) unexpected value\n"
    assert formatted =~ "panic.pt:6: Patina.Panic.panic_with_message/0\n"

    assert_raise Patina.Core.Panic, "unexpected value", &Patina.Panic.panic_with_message/0
  end

  test "todo! raises a Patina.Core.Panic exception with a not-implemented message" do
    formatted = format_panic(&Patina.Panic.todo_without_message/0)
    assert formatted =~ "** (Patina.Core.Panic) not yet implemented\n"
    assert formatted =~ "panic.pt:10: Patina.Panic.todo_without_message/0\n"

    formatted = format_panic(&Patina.Panic.todo_with_message/0)
    assert formatted =~ "** (Patina.Core.Panic) not yet implemented: handle the value\n"
    assert formatted =~ "panic.pt:14: Patina.Panic.todo_with_message/0\n"
  end

  test "panic! and todo! diverge in let-else" do
    assert 5 == Patina.Panic.panic_in_let_else(%{1 => 5})
    assert 5 == Patina.Panic.todo_in_let_else(%{1 => 5})

    formatted = format_panic(fn -> Patina.Panic.panic_in_let_else(%{}) end)
    assert formatted =~ "** (Patina.Core.Panic) missing key\n"
    assert formatted =~ "panic.pt:18: Patina.Panic.panic_in_let_else/1\n"

    formatted = format_panic(fn -> Patina.Panic.todo_in_let_else(%{}) end)
    assert formatted =~ "** (Patina.Core.Panic) not yet implemented\n"
    assert formatted =~ "panic.pt:23: Patina.Panic.todo_in_let_else/1\n"
  end

  test "inside a loop, the panic raises from the loop's helper function" do
    formatted = format_panic(fn -> Patina.Panic.panic_in_loop(3) end)
    assert formatted =~ "** (Patina.Core.Panic) reached the limit\n"
    assert formatted =~ ~s'panic.pt:32: Patina.Panic."panic_in_loop$patina_loop_1"/2\n'

    # The helper of an inner loop is called from the helper of the outer loop.
    formatted = format_panic(fn -> Patina.Panic.panic_in_nested_loop(1) end)
    assert formatted =~ "** (Patina.Core.Panic) reached the limit\n"
    assert formatted =~ ~s'panic.pt:42: Patina.Panic."panic_in_nested_loop$patina_loop_2"/3\n'
    assert formatted =~ ~s'panic.pt:41: Patina.Panic."panic_in_nested_loop$patina_loop_1"/2\n'
  end

  test "erl_error formats the panic" do
    {reason, stacktrace} =
      try do
        Patina.Panic.panic_with_message()
      catch
        :error, reason -> {reason, __STACKTRACE__}
      end

    formatted = IO.chardata_to_string(:erl_error.format_exception(:error, reason, stacktrace))
    assert formatted =~ "exception error: panic: unexpected value\n"
  end
end
