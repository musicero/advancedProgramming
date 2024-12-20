module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ remove 2 [1,2,3,4,5]

remove :: Int -> [Int] -> [Int]
remove _ [] = []
remove 0 (_:xs) = xs
remove n (x:xs) = x : remove (n-1) xs
