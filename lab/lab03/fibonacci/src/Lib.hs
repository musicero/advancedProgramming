module Lib
    ( fib
      fib2
    ) where

-- | fib 8 is 21
-- >>> fib 8

fib :: (Integral a) => a -> a
fib 0 = 0
fib 1 = 1
fib n = fib (n-1) + fib (n-2)

fibs::[Integer]
fibs = 0 : 1 : next fibs
  where
    next (a : t@(b:_)) = (a+b) : next t
:wq
fib2 :: (Integer a) => a -> a
fib2 n = fibs !! n
