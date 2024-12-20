module Lib (
  addAge,
  addNumber,
  someFunc,
  Age
) where

-- type Age = Int
-- addAge :: Age -> Age -> Age
-- addAge x y = x + y

addAge :: Age -> Age -> Age
addAge (Age x x2) (Age y) = Age (x+y)
newtype Age = Age Int Int deriving (Show)


-- here I use the Num type class (kind of like cpp templates)
addNumber :: Num a => a -> a -> a
addNumber x y = x + y

someFunc :: IO ()
someFunc = do 
  putStrLn "What is your name?"
  name <- getLine
  putStrLn "and what is your age?"
  ageStr <- getLine

  let age = read ageStr :: Int
  

  let futureAge = addAge (Age age) (Age 10)

  putStrLn $ "Hello " ++ name ++ ", in 10 years you will be " ++  show futureAge


helloworld :: IO ()
helloworld = print "hello world!"
  


