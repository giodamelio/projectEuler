defmodule ProjectEulerTest do
  use ExUnit.Case, async: true

  test "Problem 1" do
    assert ProjectEuler.P0001.solve(1000) == 233168
  end

  test "Problem 2" do
    assert ProjectEuler.P0002.solve(4000000) == 4613732
  end

  test "Problem 3" do
    assert ProjectEuler.P0003.solve(600851475143) == 6857
  end
end
