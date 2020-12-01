-- yes, the haskell answer is short, but finding it was certainly not!!
import Data.String

type Tuple = (String, String)

toTuple :: String -> Tuple
toTuple line = (take 3 line, drop 4 line)

first (x, _) = x
second (_, x) = x

pathTree :: [Tuple] -> String -> String -> [String]
pathTree list end cur
	| cur == end       = [cur]
	| length finds > 0 = cur:finds
	| otherwise        = []
	where finds = concatMap (pathTree list end) (map second (filter ((==) cur . first) list))

path :: String -> [Tuple] -> [String]
path to list = pathTree list to "COM"

main::IO()
main = interact main'

main' :: String -> String
main' text =
	show $ l youPath + l sanPath - (2 * l (t2l youPath sanPath)) - 2
	where
		youPath = pathTree (map toTuple (lines text)) "YOU" "COM"
		sanPath = pathTree (map toTuple (lines text)) "SAN" "COM"
		l = length

t2l [] [] = []
t2l _ []  = []
t2l [] _  = []
t2l (a:ax) (b:bx)
	| a == b = (a, b):t2l ax bx
	| a /= b = []

