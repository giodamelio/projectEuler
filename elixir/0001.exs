IO.puts 1..999

  # Filter out multiples of 3 and 5
 |> Enum.filter(fn x ->
   rem(x, 3) == 0 or rem(x, 5) == 0
 end)

 # Sum the numbers
 |> Enum.sum

