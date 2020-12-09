import System.IO ( isEOF )
import Control.Monad ( filterM )


solve :: [Int] -> Int -> Int
solve xs n = fst $ head $
    filter (not . cond) $
    foldr conv [] [0..((length xs) - n - 1)]
    where
        conv :: Int -> [(Int, [Int])] -> [(Int, [Int])]
        conv i acc = (head $ drop (n+i) xs, (take n $ drop i xs)):acc
        cond (x, xs) = [a * b | a <- xs,
                                b <- xs,
                                ((a + b) == x) && (a /= b)] /= []

main = do
    lines <- getLines
    let numbers = map (read :: String -> Int) lines
        in print $ solve numbers 25

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
