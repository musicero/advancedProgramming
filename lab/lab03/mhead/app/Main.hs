module Main (main) where

import Lib

main :: IO ()
main = do
  let headelm = mhead1 [1,2,3]
  print headelm
