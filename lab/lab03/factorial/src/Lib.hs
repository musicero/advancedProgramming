module Lib
    ( mfact
    ) where

-- | mfact 6 is 720
-- >>> mheads 6

-- Can take both Int and Integer types
mfact :: (Integral a) => a -> a
mfact 0 = 1
mfact x = x * mfact (x-1)




