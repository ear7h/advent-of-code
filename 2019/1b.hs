
atoi x = read x::Int

fn :: Int -> Int
fn x = -2 + quot x 3::Int

fn' :: Int -> Int
fn' x
	| fn x <= 0 = 0
	| otherwise = (fn x) + (fn' (fn x))

main :: IO()
main = interact $ show . sum . map (fn' . atoi) . lines
