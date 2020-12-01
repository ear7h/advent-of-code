
decSeq :: Int -> Bool
decSeq x
	| x == 0 = True
	| otherwise = d0 >= d1 && decSeq (x `quot` 10)
	where
		d0 = x `mod` 10
		d1 = (x `quot` 10) `mod` 10

dubs :: Int -> Bool
dubs x
	| x == 0 = False
	| (d2 == d1 && d1 == d0) = dubs (nextNot (x `quot` 10) d0)
	| otherwise = (d1 == d0) || (dubs (x `quot` 10))
	where
		d0 = x `mod` 10
		d1 = (x `quot` 10) `mod` 10
		d2 = (x `quot` 100) `mod` 10

nextNot :: Int -> Int -> Int
nextNot n digit
	| n == 0              = n
	| n `mod` 10 == digit = nextNot (n `quot` 10) digit
	| otherwise           = n


canBePass :: Int -> Bool
canBePass x = (dubs x) && (decSeq x)

main :: IO()
main = interact main'

main' :: String -> String
main' _ = show $ length $ filter canBePass [284639..748759]
