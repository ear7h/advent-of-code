
main = do
    file <- readFile "input.txt"
    input <- return $ map read (lines file)
    print $
        head $
        filter ((== 2020) . fst)
        [(a + b, a * b) | a <- input, b <- input]
