import Control.Monad
import Data.Functor
import Data.List
import Data.List.Split

parseInput :: String -> IO ([(Int, Int)], [[Int]])
parseInput filePath = do
  text <- readFile filePath
  let (ordering, sequences) = liftA2 (,) head last . splitOn [""] . lines $ text
  pure
    ( map (liftA2 (,) head last . map read . splitOn "|") ordering,
      map (map read . splitOn ",") sequences
    )

part1 :: ([(Int, Int)], [[Int]]) -> Int
part1 (ordering, sequences) =
  sum . map mid . filter ordered $ sequences
  where
    mid xs = xs !! (length xs `div` 2)
    ordered pages =
      all
        (`elem` ordering)
        [ (vi, vj)
          | (i, vi) <- zip [0 ..] pages,
            (j, vj) <- zip [0 ..] pages,
            i < j
        ]

part2 :: ([(Int, Int)], [[Int]]) -> Int
part2 (ordering, sequences) =
  sum . map (mid . sortBy compare') . filter (not . ordered) $ sequences
  where
    mid xs = xs !! (length xs `div` 2)
    compare' x y
      | x == y = EQ
      | (x, y) `elem` ordering = LT
      | otherwise = GT
    ordered pages =
      all
        (`elem` ordering)
        [ (vi, vj)
          | (i, vi) <- zip [0 ..] pages,
            (j, vj) <- zip [0 ..] pages,
            i < j
        ]

main = do
  putStr "Part 1: " *> parseInput "inputs/input-05.txt" >>= print . part1
  putStr "Part 2: " *> parseInput "inputs/input-05.txt" >>= print . part2
