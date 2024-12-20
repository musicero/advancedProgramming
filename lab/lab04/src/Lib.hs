module Lib
    ( decodeMessage
      decodeMessageImproved
    ) where

-- | Decode an intergalactic message from a string.
-- The message is a sequence of integers separated by spaces.
--
-- >>> decodeMessage "5 5 5 8 1 2 3 4 9 8 2 3 4 1"
-- Nothing
--
-- >>> decodeMessage "5 5 5 8 1 2 3 4 9 8 2 3 4 9"
-- Nothing
--
-- >>> decodeMessage "5 5 5 8 1 2 3 4 9 8 2 3 4 8"
-- Just 3
-- 
-- >>> decodeMessage "5 5 5 1 2 3 4 8 2 3"
-- Nothing
decodeMessage :: String -> Maybe Int
decodeMessage msg 
  | count min pmsg /= 1 = Nothing
  | count max pmsg /= 1 = Nothing
  | odd (min+max) = Nothing
  | otherwise = Just $ count  ((min+max) `div` 2) pmsg
  where pmsg = map read (words msg)
        min = minimum pmsg
        max = maximum pmsg

count :: Int -> [Int] -> Int
count x xs = length (filter (==x) xs)



-- | Decode an intergalactic message from a string.
-- The message is a sequence of integers separated by spaces.
-- This is an improved version of the previous function, with a more
-- informative error messages.
--
-- >>> decodeMessageImproved "5 5 5 8 1 2 3 4 9 8 2 3 4 1"
-- Left "Communication interference detected: minimum number not Unique"
--
-- >>> decodeMessageImproved "5 5 5 8 1 2 3 4 9 8 2 3 4 9"
-- Left "Communication interference detected: maximum number not Unique"
--
-- >>> decodeMessageImproved "5 5 5 8 1 2 3 4 9 8 2 3 4 8"
-- Right 3
--
-- >>> decodeMessageImproved "5 5 5 1 2 3 4 8 2 3"
-- Left "Communication interference detected: midPoint not even"
-- decodeMessageImproved :: String -> Either String Int
-- decodeMessageImproved msg 
--   | count min pmsg /= 1 = Left $ err ++ "minimum number not Unique"
--   | count max pmsg /= 1 = Left $ err ++ "maximum number not Unique"
--   | odd (min+max)  = Left $ err ++ "midPoint not even"
--   | otherwise = Right $ count (div (min+max) 2) pmsg
--   where pmsg = map read (words msg) :: Int
--         min = minimum pmsg
--         max = maximum pmsg
--         err = "Communication interference detected: "


