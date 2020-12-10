import System.IO ( isEOF )
import Data.List ( sort )


-- solve :: [Int] -> Int
solve xs =
    ((length $ filter (1==) diffs) * (length $ filter (3==) diffs))
    where
        sorted = sort ((3 + (foldr max 0 xs)):0:xs)
        diffs = map (\(a, b) -> b - a) $ zip sorted (drop 1 sorted)

main = do
    lines <- getLines
    let numbers = map (read :: String -> Int) lines
        in print $ solve numbers

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
