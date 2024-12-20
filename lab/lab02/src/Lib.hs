module Lib
    ( 
    mreverse,
    mulTable,
    fmtprint,
    oldestStudents,
    oldestStudentsFast
    ) where

import Data.List()

--- Task 1 ---

-- | mreverse is my own implementation of list reversal
-- >>> mreverse "Hello"
-- "olleH"

mreverse :: [a] -> [a]
mreverse [] = []
mreverse (x:xs) = mreverse xs ++ [x]


--- Task 2 ---

-- | fmtprint prints with padding to the left
-- >>> fmtprint 7
-- "   7"
fmtprint :: Int -> String
fmtprint x
  | x < 10 = "   " ++ show x
  | x < 100 = "  " ++ show x
  | otherwise = " " ++ show x

-- | multiplication-table using double list comprehension
mulTable :: Int -> IO ()
mulTable n = 
  let printRow row = putStrLn $ concatMap fmtprint row
  in mapM_ printRow [[(x*y) | x <- [1..n]] | y <- [1..n]]


--- Task 3 --- 

-- | Returns last word as an int
-- >>> getAge "Linus Torvalds 54"
-- 54
getAge :: String -> Int
getAge line = read $ last (words line)

oldestStudents :: String -> IO ()
oldestStudents file = do
  fileContent <- readFile file
  let contentLines = lines fileContent
  let contentAges = map getAge contentLines
  let maxAge = maximum contentAges
  let filteredAges = filter (==maxAge) contentAges
  print $ length filteredAges
  

--- Task 4 ---

-- Expected time complexity with both 
-- one and two traversals will be O(n)
-- but doing it with one traversal will
-- probably be faster :)

mysum :: (Int, Int) -> Int -> (Int, Int)
mysum (maxNumber, count) n 
  | n > maxNumber = (n, 1)
  | n == maxNumber = (maxNumber, count+1) 
  | otherwise = (maxNumber, count)

oldestStudentsFast :: String -> IO ()
oldestStudentsFast file = do
  fileContent <- readFile file
  let contentLines = lines fileContent
  let contentAges = map getAge contentLines
  let abc = foldl mysum (0,0) contentAges
  putStrLn $ "There is " 
          ++ show (snd abc) 
          ++ " students of age " 
          ++ show (fst abc)
