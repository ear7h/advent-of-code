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

data Dir =  U Int | D Int | L Int | R Int

str2dir :: String -> Dir
str2dir s
	| startsWith s "U" = U (atoi $ drop 1 s)
	| startsWith s "D" = D (atoi $ drop 1 s)
	| startsWith s "L" = L (atoi $ drop 1 s)
	| startsWith s "R" = R (atoi $ drop 1 s)

type Point = (Int, Int)

dirs2lines :: [Dir] -> [(Point, Point)]
dirs2lines x = drop 0 $ dirs2lines' (0, 0) x

dirs2lines' :: Point -> [Dir] -> [(Point, Point)]
dirs2lines' _ [] = []
dirs2lines' (x, y)  (d:ds) =
	((x, y), next): dirs2lines' next ds
		where next = case d of
			L n -> (x - n, y)
			R n -> (x + n, y)
			U n -> (x, y + n)
			D n -> (x, y - n)

-- ((8,5),(3,5)),((0,0),(0,7))
intersection :: (Point, Point) -> (Point, Point) -> Maybe Point
intersection ((a1x, a1y), (a2x, a2y)) ((b1x, b1y), (b2x, b2y))
	| minbx >= (max a1x a2x) ||
		maxbx <= (min a1x a2x) ||
		minby >= (max a1y a2y) ||
		maxby <= (min a1y a2y) = Nothing
	| a1y == a2y && b1x == b2x = Just (b1x, a1y)
	| a1x == a2x && b1y == b2y = Just (a1x, b1y)
	where
		minbx = min b1x b2x
		maxbx = max b1x b2x
		minby = min b1y b2y
		maxby = max b1y b2y

intersectionP :: (Point, Point) -> Point-> Bool
intersectionP ((l1x, l1y), (l2x, l2y)) (px, py) =
	(l1x == l2x && l1x == px && minly <= py && maxly >= py) ||
	(l1y == l2y && l1y == py && minlx <= px && maxlx >= py)
	where
		maxly = max l1y l2y
		minly = min l1y l2y
		maxlx = max l1x l2x
		minlx = min l1x l2x

intersections :: [(Point, Point)] -> [(Point, Point)] -> [Point]
intersections _ [] = []
intersections [] _ = []
intersections ax (b:bx) = (intersections' ax b) ++ (intersections ax bx)

intersections' :: [(Point, Point)] -> (Point, Point) -> [Point]
intersections' [] _ = []
intersections' (a:ax) b = case intersection a b of
	Just p -> p:intersections' ax b
	Nothing -> intersections' ax b

distPath :: [(Point, Point)] -> Point -> Maybe Int
distPath [] _ = Nothing
distPath (l:rest) p
	| intersectionP l p = Just $ dist l1 p
	| otherwise = case distPath rest p of
		Just n -> Just $ (dist l1 l2) + n
		Nothing -> Nothing
	where
		(l1, l2) = l

dist :: Point -> Point -> Int
dist (ax, ay) (bx, by) = (abs (ax - bx)) + (abs (ay - by))

minM :: Ord a => a -> Maybe a -> a
minM x y = case y of
	Just n -> min x n
	Nothing -> x

addM :: Num a => Maybe a -> Maybe a -> Maybe a
addM (Just x) (Just y) = Just (x + y)
addM (Just x) Nothing = Just x
addM Nothing (Just y) = Just y
addM Nothing Nothing = Nothing

filterM :: [Maybe a] -> [a]
filterM [] = []
filterM (x:xs) = case x of
	Just xx -> xx:(filterM xs)
	Nothing -> filterM xs

main :: IO()
main = interact main'

main' :: String -> String
main' s = show $
	foldl minM 999999 $
		drop 1 $
			map (\x -> (distPath wire1 x) `addM` (distPath wire2 x)) (intersections wire1 wire2)
	where
		wires = lines s
		wire1 = dirs2lines $ map str2dir $ splitAll (wires!!0) ","
		wire2 = dirs2lines $ map str2dir $ splitAll (wires!!1) ","

