import System.IO ( isEOF )
import Control.Monad ( filterM )
import Data.List ( foldl1, foldl' )
import Debug.Trace ( trace )


solve :: [Int] -> Int -> Int
solve xs n = (\xs -> (foldl1 min xs) + (foldl1 max xs)) $
    head $
    filter ((n == ) . foldl' (+) 0) $
    [ slice xs i j | i <- [0..],
                     j <- [0..i]]
    where
        slice xs i j =
            take (i-j) $ drop i xs

main = do
    lines <- getLines
    let numbers = map (read :: String -> Int) lines
        in print $ solve numbers 2089807806

getLines :: IO [String]
getLines = do
    eof <- isEOF
    if not eof
    then do
        x <- getLine
        xs <- getLines
        return (x:xs)
    else do
        return []
