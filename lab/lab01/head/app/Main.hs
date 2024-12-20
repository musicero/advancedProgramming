module Main (main) where

import Lib

main :: IO ()
main = do
  let test1 = mhead [4, 2, 3]
  let test2 = mhead ['a', 'b', 'c']
  let test3 = mhead "Hello"

  putStrLn ("test1: " ++ show test1)
  putStrLn ("test2: " ++ show test2)
  putStrLn ("test3: " ++ show test3)
