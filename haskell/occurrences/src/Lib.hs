module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn $ show $ mostFrequent [1, 2,3, 3, 4,4,5]


-- have to import these
-- import Data.List (group, sort, maximumBy)

mostFrequent :: [Int] -> Int
mostFrequent xs = 
  fst -- return value
  $ maximumBy (\x y -> compare (snd x) (snd y)) -- compare length
  $ map (\ys ->(head ys, length ys)) -- for each sublist, make a tuple (value, length)
  $ group -- groups into sublists
  $ sort xs -- this will take care of the edge case of equal count, different values
