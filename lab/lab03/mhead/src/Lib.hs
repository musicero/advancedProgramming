module Lib 
  ( mhead1,
    mhead2,
    mhead3,
    mhead4,
    mhead5,
    mhead6
  ) where

-- | mhead1 [1,2,3] is 1
-- >>> mhead1 [1,2,3]
--
mhead1 :: [a] -> a
mhead1 [] = error "empty list"
mhead1 (a:_) = a


mhead2 :: [a] -> a
mhead2 xs
  | not (null xs) = xs !! 0
  | otherwise = error "empty list"

mhead3 :: [a] -> a
mhead3 xs = if null xs then error "empty list" else xs !! 0

mhead4 :: [a] -> a
mhead4 xs = 
  let 
    isEmpty = null xs
    fstVal = xs !! 0
  in 
    if not isEmpty
      then fstVal
      else error "empty list"

mhead5 :: [a] -> a
mhead5 xs =
  if not isEmpty
    then fstVal
    else error "empty list"
  where
    isEmpty = null xs
    fstVal = xs !! 0

mhead6 :: [a] -> a
mhead6 xs = case xs of [] -> error "empty list"
                       (x:_) -> x
