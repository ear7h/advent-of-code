
atoi x = read x::Int

fn :: Int -> Int
fn x = -2 + quot x 3::Int

main :: IO()
main = interact $ show . sum . map (fn . atoi) . lines
