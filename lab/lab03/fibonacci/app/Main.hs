module Main (main) where

import Lib

main :: IO ()
main = print $ fib (8 :: Int)
