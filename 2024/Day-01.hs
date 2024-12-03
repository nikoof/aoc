import Data.Functor
import Data.List

parseInput :: String -> IO ([Int], [Int])
parseInput filePath = do
  text <- readFile filePath
  let entries = map (map read . words) (lines text) :: [[Int]]
  pure (map head entries, map last entries)

part1 :: ([Int], [Int]) -> Int
part1 (left, right) = sum $ zipWith (\x y -> abs (x - y)) (sort left) (sort right)

part2 :: ([Int], [Int]) -> Int
part2 (left, right) = sum $ map similarity left
  where
    similarity x = x * length (filter (== x) right)

main = do
  putStr "Part 1: " *> parseInput "inputs/input-01.txt" >>= print . part1
  putStr "Part 2: " *> parseInput "inputs/input-01.txt" >>= print . part2
