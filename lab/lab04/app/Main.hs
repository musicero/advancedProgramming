module Main (main) where

import Lib

main :: IO ()
main = undefined

    -- msg <- getContents
    -- case decodeMessage msg of
    --     Just n -> putStrLn $ "The message is " ++ show n ++ " characters long."
    --     Nothing -> putStrLn "The message cannot be decoded. Interference detected."
    --
    -- msg2 <- getContents
    -- print msg2
    -- case decodeMessageImproved msg2 of
    --     Right n -> putStrLn $ "The message is " ++ show n ++ " characters long."
    --     Left err -> putStrLn err
