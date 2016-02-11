defmodule ProjectEuler.Mathlib do
  @doc """
  Create a lazy stream of the Fibonacci sequence
  """
  def fib do
    Stream.unfold({0, 1}, fn {a, b} -> {a, {b, a + b}} end)
  end
end
