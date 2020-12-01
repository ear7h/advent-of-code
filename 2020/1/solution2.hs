
main = do
    file <- readFile "input.txt"
    input <- return $ map read (lines file)
    print $
        head $
        filter ((== 2020) . fst)
        [(a + b + d, a * b * d) | a <- input,
                                  b <- input,
                                  d <- input]
