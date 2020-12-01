-- yes, the haskell answer is short, but finding it was certainly not!!
import Data.String

type Tuple = (String, String)

toTuple :: String -> Tuple
toTuple line = (take 3 line, drop 4 line)

first (x, _) = x
second (_, x) = x

countTree :: Int -> [Tuple] -> Tuple -> Int
countTree depth list (here, to) =
	 (+) depth $ sum $ map (countTree (depth + 1) list) (filter ((==) to . first) list)

count :: [Tuple] -> Int
count list = countTree 0 list ("", "COM")

-- I have no idea how the operator precedence works
main::IO()
main = interact $ show . count . map toTuple . lines
