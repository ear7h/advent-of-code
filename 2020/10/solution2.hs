import System.IO ( isEOF )
import Data.List ( sort )


solve :: [Int] -> Integer
solve xs = foldl (*) 1 $
           map (f . toInteger . length) split
    where
        sorted = sort ((3 + (foldr max 0 xs)):0:xs)
        diffs = map (\(a, b) -> b - a) $ zip sorted (drop 1 sorted)
        split = splitOn 3 diffs

splitOn :: Eq a => a -> [a] -> [[a]]
splitOn delim xs = reverse $ foldl f [[]] xs
    where
        f acc@(a:cc) x = if x == delim
                  then []:acc
                  else (a ++ [x]):cc



-- combinatorial guessing...
--
--  3 3
f 0 = 1
--  3 1 3
f 1 = 1
--  3 1 1 3
--  3   2 3
f 2 = 2
--  3 1 1 1 3
--  3   2 1 3 => f 2
--  3 1   2 3
--  3     3 3 => 2
f 3 = 4
--  3 1 1 1 1 3
--  3   2 1 1 3
--  3 1   2 1 3
--  3     3 1 3 => f 3
--  3 1 1   2 3
--  3   2   2 3
--  3 1     3 3 => 3
f 4 = 7
-- hmmm...
f n = (n-1)  + f (n -1)


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
