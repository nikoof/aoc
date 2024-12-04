import Control.Monad
import Data.Functor
import Data.List

parseInput :: String -> IO [[Char]]
parseInput filePath = readFile filePath <&> lines

windows :: Int -> [a] -> [[a]]
windows n = filter ((== n) . length) . map (take n) . tails

rotate45 :: Int -> [[a]] -> [[a]]
rotate45 direction matrix =
  map (map snd)
    . groupBy (\(k1, _) (k2, _) -> k1 == k2)
    . sortOn fst
    $ concatMap
      (\(i, row) -> [(i + direction * j, val) | (j, val) <- zip [0 ..] row])
      (zip [0 ..] matrix)

part1 :: [[Char]] -> Int
part1 grid =
  sum . map (\f -> countWords . f $ grid) $ [id, transpose, rotate45 1, rotate45 (-1)]
  where
    countWords = sum . map (length . filter ((||) <$> (== "XMAS") <*> (== "SAMX")) . windows 4)

part2 :: [[Char]] -> Int
part2 grid =
  length $
    filter
      xmas
      [ (i, j)
        | i <- [0 .. length grid - 3],
          j <- [0 .. length grid - 3]
      ]
  where
    xmas :: (Int, Int) -> Bool
    xmas (r, c) =
      let pattern = map (\(i, j) -> grid !! (r + i) !! (c + j)) [(0, 0), (1, 1), (2, 2), (0, 2), (1, 1), (2, 0)]
       in pattern `elem` ["MASMAS", "SAMSAM", "MASSAM", "SAMMAS"]

main = do
  putStr "Part 1: " *> parseInput "inputs/input-04.txt" >>= print . part1
  putStr "Part 2: " *> parseInput "inputs/input-04.txt" >>= print . part2
