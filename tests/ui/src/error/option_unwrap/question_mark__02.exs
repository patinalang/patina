ExUnit.start()

defmodule QuestionMark02Test do
  use ExUnit.Case

  alias Patina.QuestionMark02.{Person, PhoneNumber}

  # `Person` and `Job` are transparent, so a `Person` is an `Option<Option<PhoneNumber>>`:
  # `Some(Some(phone))` is the bare record, `Some(None)` is `{:some, nil}` and `None` is `nil`.
  defp phone(area_code),
    do: PhoneNumber."$patina_constructor__PhoneNumber"(area_code, 439_222_222)

  test "main's assert_eq! passes" do
    assert {} == Patina.QuestionMark02.main()
  end

  test "work_phone_area_code returns the area code when every field is set" do
    assert 61 == Person.work_phone_area_code(phone(61))
  end

  test "work_phone_area_code returns None when the area code is None" do
    assert nil == Person.work_phone_area_code(phone(nil))
  end

  test "work_phone_area_code returns None when the phone number is None" do
    assert nil == Person.work_phone_area_code({:some, nil})
  end

  test "work_phone_area_code returns None when the job is None" do
    assert nil == Person.work_phone_area_code(nil)
  end
end
