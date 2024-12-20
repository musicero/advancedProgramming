module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ filter' even [1, 2, 3, 4] 

filter' :: (a -> Bool) -> [a] -> [a]
filter' _ [] = []
filter' p (x:xs) 
  | p x = x:r
  | otherwise = r
  where r = filter' p xs


filter2 :: 

