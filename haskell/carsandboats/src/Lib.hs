module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ mostExpensive 5000 [1000,2000,3000] [1000,2000,3000]

mostExpensive :: Int -> [Int] -> [Int] -> Int
mostExpensive budget cars boats 
  | null inBudget = -1
  | otherwise = maximum inBudget
  where inBudget = filter (<=budget) $ map (+) cars <*> boats

  -- Path: app/Main.hs

