isPalindrome :: (Integral a, Show a) => a -> Bool
isPalindrome num =
    let string = show num
    in string == (reverse string)

main :: IO ()
main = putStrLn $ show $ maximum $ [x*y | x <- [1 .. 999], y <- [1 .. 999], isPalindrome (x * y)]
