fibSeq :: (Integral a) => [a]
fibSeq = 1 : 1 : zipWith (+) fibSeq (tail fibSeq)

main :: IO ()
main = putStrLn $ show $ sum $ filter even $ takeWhile (<4000000) $ fibSeq
