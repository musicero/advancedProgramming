module Main (main) where

import Lib (Age, addAge)

main :: IO ()
main = print $ addAge (Age 3) (Age 5)
