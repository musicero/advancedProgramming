module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ filter3 even [1..10]


filter2 :: (a -> Bool) -> [a] -> [a]
filter2 p xs = xs >>= (\x -> if p x then pure x else [])

-- filter with foldr
filter3 :: (a -> Bool) -> [a] -> [a]
filter3 p xs = foldr (\x acc -> if p x then x:acc else acc) [] xs


