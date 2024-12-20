module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ swap [1,2,3,4]

swap :: [a] -> [a]
swap [] = []
swap [x] = [x]
swap (x:y:xs) = y:x:(swap xs)
