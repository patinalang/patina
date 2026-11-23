ExUnit.start()

defmodule ReceiveTest do
  use ExUnit.Case
  alias Patina.ReceiveTest, as: Receive

  test "selective receive leaves unmatched messages in the mailbox" do
    send(self(), :a)
    send(self(), 42)
    assert Receive.take_int() == 42
    assert Receive.take_atom() == :a
  end

  test "timeout fires only when no matching message is queued" do
    assert Receive.take_int_or_timeout() == -1
    send(self(), 17)
    assert Receive.take_int_or_timeout() == 17
    assert Receive.only_timeout() == 7
  end

  test "extern erlang calls round trip through a spawned process" do
    process = spawn(Receive, :echo, [])
    send(process, {:ping, self()})
    assert_receive :pong
  end

  test "receive cooperates with loop break markers and mutable variables" do
    for message <- [3, 5, -2, :stop], do: send(self(), message)
    assert Receive.sum_until_stop() == 6
  end

  test "unsafe ADT patterns guard untrusted fields" do
    send(self(), {:value, :bad})
    send(self(), {:value, 23})
    assert Receive.unsafe_value() == 23
    assert Receive.unsafe_value() == -1
    assert_receive {:value, :bad}
  end

  test "guards and nested or patterns select matching messages" do
    send(self(), {:left, -1})
    send(self(), {:right, 8})
    assert Receive.guarded_or() == 8
    assert_receive {:left, -1}
  end

  test "timeout branches join mutable variables" do
    assert Receive.merge_timeout() == 2
    send(self(), 11)
    assert Receive.merge_timeout() == 11
    assert Receive.unchanged_timeout() == 1
    send(self(), 12)
    assert Receive.unchanged_timeout() == 12
  end

  test "receive supports closures and diverging branches" do
    send(self(), 5)
    assert Receive.closure_receive(3) == 8
    assert Receive.diverging_timeout() == -1
    send(self(), 6)
    assert Receive.diverging_timeout() == 6
  end

  test "after is contextual and receive is a raw identifier" do
    assert Receive.durations() == {2000, 180000}
    assert Receive.contextual_after() == 4
    assert Receive.raw_identifiers() == 9
  end
end
