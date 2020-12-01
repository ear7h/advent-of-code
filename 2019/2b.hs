startsWith :: String -> String -> Bool
startsWith _ "" = True
startsWith "" _ = False
startsWith (x:xs) (y:ys) = x == y && startsWith xs ys

findFirst :: String -> String -> Maybe Int
findFirst s d = findFirst' 0 s d

findFirst' :: Int -> String -> String -> Maybe Int
findFirst' n s d
	| d == ""        = Nothing
	| s == ""        = Nothing
	| startsWith s d = Just n
	| otherwise      = findFirst' (n+1) (drop 1 s) d


findAll :: String -> String -> [Int]
findAll _ "" = []
findAll s d = case findFirst s d of
	Nothing -> []
	Just n -> n:(map (+(n+length d)) (findAll (drop (n+(length d)) s) d))


splitFirst :: String -> String -> [String]
splitFirst s d = case findFirst s d of
	Nothing -> [s]
	Just n -> [x, xs]
		where (x, _:xs) = splitAt n s

splitAll :: String -> String -> [String]
splitAll s d
	| length sf == 1 = sf
	| length sf == 2 = (head sf):(splitAll (sf!!1) d)
	where sf = splitFirst s d

atoi :: String -> Int
atoi x = read x::Int

listUpdate :: [a] -> (Int, a) -> [a]
listUpdate xs (i, x) = take i xs ++ [x] ++ drop (i+1) xs

intCode :: [Int] -> [Int]
intCode mem = intCode' (0, mem)

intCode0 :: [Int] -> Int
intCode0 mem = (intCode mem)!!0

intCode' :: (Int, [Int]) -> [Int]
intCode' (pc, mem) = case mem !! pc of
	99 -> mem
	1 -> intCode' $ opAdd (pc, mem)
	2 -> intCode' $ opMul (pc, mem)
	otherwise -> error "unknown opcode"

opAdd :: (Int, [Int]) -> (Int, [Int])
opAdd (pc, mem) = (pc+4, listUpdate mem (dst, (mem!!src1 + mem!!src2)))
	where
		src1 = mem!!(pc+1)
		src2 = mem!!(pc+2)
		dst = mem!!(pc+3)

opMul :: (Int, [Int]) -> (Int, [Int])
opMul (pc, mem) = (pc+4, listUpdate mem (dst, ((mem!!src1) * (mem!!src2))))
	where
		src1 = mem!!(pc+1)
		src2 = mem!!(pc+2)
		dst = mem!!(pc+3)

lattice :: [a] -> [b] -> [(a, b)]
lattice (a:as) b = lattice' a b ++ lattice as b
lattice [] b = []

lattice' :: a -> [b] -> [(a, b)]
lattice' a b = map (\bb -> (a, bb)) b

main :: IO()
main = interact main'

main' :: String -> String
main' s = show $ (!!0) $ filter ((== 19690720) . (!!0) . intCode) progs
	where
		prog = map atoi (splitAll s ",")
		progs = map (setProg prog)  (lattice [0..99] [0..99])

setProg :: [Int] -> (Int, Int) -> [Int]
setProg mem (n, v) = listUpdate (listUpdate mem (1, n)) (2, v)


