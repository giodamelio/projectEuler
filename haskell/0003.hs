primeFactors :: (Integral a) => a -> [a]
primeFactors 0 = []
primeFactors num =
  case factors of
    [] -> [num]
    _  -> factors ++ primeFactors (num `div` (head factors))
  where factors = take 1 $ filter (\x -> (num `mod` x) == 0) [2 .. num-1]

main :: IO ()
main = putStrLn $ show $ last $ primeFactors 600851475143
