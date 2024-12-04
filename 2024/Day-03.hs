import Text.Regex.TDFA

parseMatches :: ([[String]], Bool) -> String -> ([[String]], Bool)
parseMatches (list, toggle) x
  | x == "do()" = (list, True)
  | x == "don't()" = (list, False)
  | otherwise = (list ++ ([getAllTextMatches (x =~ "[0-9]+") | toggle]), toggle)

parseInput :: String -> String -> IO [(Int, Int)]
parseInput regex filePath = do
  text <- readFile filePath
  let matches = getAllTextMatches (text =~ regex) :: [String]
  let numbers = fst $ foldl parseMatches ([], True) matches :: [[String]]
  pure $ map (liftA2 (,) head last . map read) numbers

part1 :: [(Int, Int)] -> Int
part1 = sum . map (uncurry (*))

part2 :: [(Int, Int)] -> Int
part2 = sum . map (uncurry (*))

main = do
  let regex1 = "mul\\([0-9]{1,},[0-9]{1,}\\)"
  let regex2 = "mul\\([0-9]{1,},[0-9]{1,}\\)|do\\(\\)|don't\\(\\)"
  putStr "Part 1: " *> parseInput regex1 "inputs/input-03.txt" >>= print . part1
  putStr "Part 2: " *> parseInput regex2 "inputs/input-03.txt" >>= print . part2
