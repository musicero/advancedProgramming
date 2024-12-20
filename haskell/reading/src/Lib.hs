module Lib
    ( someFunc
    ) where

someFunc :: IO ()
someFunc = putStrLn "someFunc"

type Name = String
stringCannotBeEmpty :: String -> Text -> Validation [String] Text
stringCannotBeEmpty prefix text
  | Data.Text.null text = Failure [prefix ++ " cannot be empty"]
  | otherwise = Success text
validateName :: Name -> Validation [String] Name
validateName text = case sequenceA $
 [stringCannotBeEmpty, stringOnlyLetters, stringFirstCap, stringLength 2]
 <*> ["Name"]
 <*> pure text of
 Success side -> Success $ Prelude.head side
 Failure f -> Failure f

