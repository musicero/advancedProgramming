module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show s 

s :: Int
s = foldl (+) 0 $
        takeWhile (< 1000) $
        filter ((flip mod) x 2==0) $
        map (*10) [0..]
