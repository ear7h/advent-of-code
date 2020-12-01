
decSeq :: Int -> Bool
decSeq x
	| x == 0 = True
	| otherwise = d0 >= d1 && decSeq (x `quot` 10)
	where
		d0 = x `mod` 10
		d1 = (x `quot` 10) `mod` 10

dubs :: Int -> Bool
dubs 0 = False
dubs x = (d1 == d0) || (dubs (x `quot` 10))
	where
		d0 = x `mod` 10
		d1 = (x `quot` 10) `mod` 10


canBePass :: Int -> Bool
canBePass x = (dubs x) && (decSeq x)

main :: IO()
main = interact main'

main' :: String -> String
main' _ = show $ length $ filter canBePass [284639..748759]
