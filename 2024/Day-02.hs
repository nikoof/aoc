import Control.Monad
import Data.Functor

parseInput :: String -> IO [[Int]]
parseInput filePath = readFile filePath <&> (map (map read . words) . lines)

validReport :: [Int] -> Bool
validReport = ((&&) <$> monotone <*> diffConstraints . map abs) . (zipWith (-) <$> tail <*> id)
  where
    monotone = (||) <$> all (< 0) <*> all (> 0)
    diffConstraints = all $ (&&) <$> (1 <=) <*> (<= 3)

part1 :: [[Int]] -> Int
part1 = length . filter validReport

part2 :: [[Int]] -> Int
part2 = length . filter valid
  where
    valid report = any validReport [take i report ++ drop (i + 1) report | i <- [0 .. length report - 1]]

main = do
  putStr "Part 1: " *> parseInput "inputs/input-02.txt" >>= print . part1
  putStr "Part 2: " *> parseInput "inputs/input-02.txt" >>= print . part2
