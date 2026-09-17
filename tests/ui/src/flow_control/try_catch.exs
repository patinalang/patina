ExUnit.start()

defmodule TryCatchTest do
  use ExUnit.Case

  alias Patina.TryCatch

  test "catches error, throw, and exit classes with optional stacktraces" do
    assert TryCatch.error_reason(fn -> :erlang.error(:plain) end) == :plain
    assert is_list(TryCatch.error_with_stack(fn -> :erlang.error(:plain) end))
    assert TryCatch.thrown(fn -> throw({:value, 1}) end) == {:value, 1}
    assert TryCatch.thrown_with_stack(fn -> throw(:value) end) == :value
    assert TryCatch.exited(fn -> exit(:gone) end) == :gone
  end

  test "catches badarg and Elixir exception structs" do
    assert TryCatch.argument_error(fn -> :erlang.error(:badarg) end)
    assert is_list(TryCatch.argument_error_stack(fn -> :erlang.error(:badarg) end))
    assert TryCatch.elixir_name(fn -> raise ArithmeticError end) == ArithmeticError
    assert %{__exception__: true} = TryCatch.elixir_reason(fn -> raise ArgumentError, "bad" end)
    assert is_list(TryCatch.elixir_stack(fn -> raise ArithmeticError end))
  end

  test "any catches and binds every exception class" do
    assert TryCatch.any_class(fn -> :erlang.error(:boom) end) == :error
    assert TryCatch.any_class(fn -> throw(:boom) end) == :throw
    assert TryCatch.any_class(fn -> exit(:boom) end) == :exit
    assert is_list(TryCatch.any_with_stack(fn -> throw(:boom) end))
    assert TryCatch.any_atom_reason(fn -> :erlang.error(:boom) end) == {:error, :boom}
    assert TryCatch.any_error_value(fn -> :erlang.error({:value, 7}) end) == {:value, 7}
    assert catch_exit(TryCatch.any_error_value(fn -> exit(:uncaught) end)) == :uncaught
    assert TryCatch.any_pinned_class(:throw, fn -> throw(:pinned) end) == :pinned
    assert TryCatch.any_aliased_class(fn -> :erlang.error(:boom) end) == :error
    assert TryCatch.any_class_alternative(fn -> throw(:alternative) end) == :alternative
  end

  test "reason positions support term patterns, guards, alternatives, and unsafe ADTs" do
    assert TryCatch.atom_reason(fn -> :erlang.error(:an_atom) end) == :an_atom
    assert TryCatch.badmatch_value(fn -> :ok = :error end) == :error
    assert TryCatch.map_reason(fn -> :erlang.error(%{kind: 7}) end) == 7
    assert TryCatch.arithmetic_reason(fn -> :erlang.error(:badarg) end) == :arithmetic
    assert TryCatch.arithmetic_reason(fn -> 1 / 0 end) == :arithmetic
    assert TryCatch.guarded_reason(fn -> :erlang.error(:wanted) end) == :wanted
    assert is_list(TryCatch.guarded_stack(fn -> :erlang.error(:wanted) end))
    assert TryCatch.unsafe_reason(fn -> throw({:my_error, 42}) end) == 42
  end

  test "after runs on successful and exceptional paths" do
    assert TryCatch.catch_only() == 7
    assert TryCatch.after_only(self()) == 8
    assert_receive :after_success
    assert TryCatch.after_error(self(), fn -> :erlang.error(:boom) end) == :boom
    assert_receive :after_error
    assert TryCatch.contextual_after() == 9
  end

  test "unmatched exceptions propagate" do
    assert catch_exit(TryCatch.error_reason(fn -> exit(:uncaught) end)) == :uncaught
  end
end
