module Main (main) where

import Lib
import System.IO

main :: IO ()
main = do
  putStrLn "==Task 1=="
  print $ mreverse [1,2,3]

  putStrLn "==Task 2=="
  mulTable 8

  putStrLn "==Task 3=="
  oldestStudents "students.txt"

  putStrLn "==Task 4=="
  oldestStudentsFast "students.txt"

