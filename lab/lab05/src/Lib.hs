module Lib
    ( countScore
    ) where

-- | Count the total score of the four dice game data.
-- This function is already implemented for you.
-- 
countScore :: String -> Int
countScore txt = sum $ map processLine (lines txt)


-- | Process a single line of the input data.
--
-- >>> processLine "4 21 10 5 4 21 13 11"
-- 5
-- >>> processLine "4 21 10 5 4 21 13 11 10"
-- 7
--
-- >>> processLine "4 21 10 5 4 21 13 11 10 10"
-- 11
--
-- >>> processLine "10 21 10 5 4 21 13 11 10"
-- 8
--
-- >>> processLine "10 21 10 5 8 20 13 11"
-- 0
-- >>> processLine "10 10 10 5 4 21 13 11 10 10 10"
-- 56
--
-- >>> processLine "8 14 16 5 8 14 16 14"
-- 9
-- 
-- >>> processLine "8 14 16 5 8 18 16 12"
-- 3

processLine :: String -> Int
--processLine line = calcLine win_nums bet_nums
processLine line = calcScore win_nums bet_nums 0
  where all_nums = map read $ words line
        win_nums = take 3 all_nums
        bet_nums = drop 3 all_nums

-- calcLine [Int] -> [Int] -> [Int]
-- calcLine wins bets = map (\bet -> calcPoints wins bet ) bets

calcScore :: [Int] -> [Int] -> Int -> Int
calcScore _ [] score = score 
calcScore wins bets score = calcScore wins bets' score'
  where bets' = tail bets
        current_num = head bets
        prev_occurences = (length $ filter (==current_num) bets) - 1
        base_points = calcPoints wins current_num
        score' = score + (base_points * 2^prev_occurences)
        


calcPoints :: [Int] -> Int -> Int
calcPoints wins bet
  | not exists = 0
  | bet < 10 = 1 * multiplier
  | bet < 20 = 2 * multiplier
  | otherwise = 4 * multiplier
    where exists = elem bet wins
          multiplier = 
            case length (filter (==bet) wins) of
              0 -> 0
              1 -> 1
              2 -> 2 
              3 -> 4
              _ -> error "calcpoints error"




