ExUnit.start()

defmodule Maps01Test do
  use ExUnit.Case
  import ExUnit.CaptureIO

  test "main looks up, updates and iterates the contacts" do
    output = capture_io(fn -> Patina.Maps01.main() end)

    lookups = """
    Calling Daniel: We're sorry, the call cannot be completed as dialed.
                Please hang up and try again.
    Calling Ashley: Hello, this is Mr. Awesome's Pizza. My name is Fred.
                What can I get for you today?
    """

    assert String.starts_with?(output, lookups)

    # `for` iterates the map in arbitrary order.
    calls =
      output
      |> String.replace_prefix(lookups, "")
      |> String.split("\n", trim: true)
      |> Enum.sort()

    assert calls == [
             "Calling Daniel: Hi! Who is this again?",
             "Calling Katie: Hi! Who is this again?",
             "Calling Laura: Hi! Who is this again?",
             "Calling Robert: Hi! Who is this again?"
           ]
  end
end
