defmodule ProjectEulerTest do
  use ExUnit.Case, async: true

  test "Problem 1" do
    assert ProjectEuler.P0001.solve(1000) == 233168
  end

  test "Problem 2" do
    assert ProjectEuler.P0002.solve(4000000) == 4613732
  end
end
