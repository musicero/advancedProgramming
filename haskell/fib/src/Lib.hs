module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ fiblazy 20

fib :: Int -> Int
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)

fiblazy :: Int -> Int
fiblazy = [0,1] ++ zipWith (+) fiblazy (tail fib)

