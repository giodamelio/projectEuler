greatestCommonDivisor :: (Integral a) => a -> a -> a
greatestCommonDivisor a 0 = abs a
greatestCommonDivisor a b = greatestCommonDivisor b (a `mod` b)

leastCommonMultiple :: (Integral a) => a -> a -> a
leastCommonMultiple a b = (a * b) `div` (greatestCommonDivisor a b)

main :: IO ()
main = putStrLn $ show $ foldl (\acc x -> leastCommonMultiple acc x) 1 $ [x | x <- [1..20]]
