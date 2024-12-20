module Lib
    ( decodeMessage
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
decodeMessage msg = undefined



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
decodeMessageImproved :: String -> Either String Int
decodeMessageImproved msg = undefined