module Main (main) where

import Lib

main :: IO ()
-- main = do
--     let factorial = mfact 6
--     print factorial

main = print $ mfact (6 :: Integer)
  
