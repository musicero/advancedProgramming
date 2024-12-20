module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ insert 3 [1,2,4,5,6]


insert :: Int -> [Int] -> [Int]
insert x [] = [x]
insert x (y:ys)
  | x < y = x:y:ys
  | otherwise = y : insert x ys











insert :: Int -> [Int] -> [Int]
insert x [] = [x]
insert x (y:ys)
  | x<y = x:y:ys
  | otherwise = y : insert x ys







