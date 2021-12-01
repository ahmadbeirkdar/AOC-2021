solve1 :: [Int] -> Int
solve1 x = length $ filter (<0) $ zipWith (-) x (tail x)