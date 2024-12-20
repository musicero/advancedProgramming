module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ read $ splitAt 4 "Hello world"


