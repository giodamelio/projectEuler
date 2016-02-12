defmodule ProjectEuler.Mathlib do
  @doc """
  Create a lazy stream of the Fibonacci sequence
  """
  def fib do
    Stream.unfold({0, 1}, fn {a, b} -> {a, {b, a + b}} end)
  end

  @doc """
  Calculates all prime factors of a number by finding a low factor
  and then recursively calculating the factors of the high factor.
  Skips all evens except 2.
  Could be further optimized by only using known primes to find factors.
  """
  def prime_factors(num , next \\ 2)
  def prime_factors(num, 2) do
    cond do
      rem(num, 2) == 0 -> [2 | prime_factors(div(num, 2))]
      4 > num          -> [num]
      true             -> prime_factors(num, 3)
    end |> Enum.uniq
  end
  def prime_factors(num, next) do
    cond do
      rem(num, next) == 0 -> [next | prime_factors(div(num, next))]
      next + next > num   -> [num]
      true                -> prime_factors(num, next + 2)
    end |> Enum.uniq
  end

  @doc """
  Check if a number is a palindrome
  """
  def is_palindrome(num) do
    string = Integer.to_string num
    string == String.reverse string
  end
end
