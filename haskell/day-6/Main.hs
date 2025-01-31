import System.IO
import Data.Data
import Data.List
import qualified Data.Set as Set
import Control.Concurrent

guard = '^'
obstruction = '#'
guardStartingDirection = DUp
type Position = (Int, Int)

data Guard = Guard { guardPosition :: Position, guardDirection :: Direction }

main :: IO ()
main = do
  input <- lines <$> readFile "small.txt"

  let guardPos = findGuardPosition $ indexLst input
      guard' = case guardPos of
        Just pos -> Guard pos guardStartingDirection
        Nothing  -> error "No guard found in the input grid."

      stepsTaken = countTotalSteps ([], guard') input
      -- Remove duplicates
      distinctPositions = length . Set.toList $ Set.fromList stepsTaken

  let removeGuard = map (\ch -> if ch == guard then '.' else ch)
      input' = map removeGuard input

  let input''      = indexLst $ map indexLst input'
  mapM_ (printWithMovingGuard input'') [reverse stepsTaken]

  putStrLn $ "The guard visited " ++ show distinctPositions ++ " distinct positions"

indexLst = zip [0..]

printWithMovingGuard :: [(Int, [(Int, Char)])] -> [Position] -> IO ()
printWithMovingGuard _ [] = return ()
printWithMovingGuard grid (step:stepsRemaining) = do
  putStr "\ESC[2J"
  let putGuard = map (\col -> if fst col == snd step then guard else snd col)
      grid'    = map (\row -> if fst row == fst step then putGuard $ snd row else map snd $ snd row) grid :: [[Char]]
  mapM_ print grid'
  threadDelay 100000
  printWithMovingGuard grid stepsRemaining

type Steps = Int
-- Count the total steps until the guard leaves the area.
countTotalSteps :: ([Position], Guard) -> [[Char]] -> [Position]
countTotalSteps (steps, guard) grid = case dir of
    DUp    -> if fst lookaheadPosition <= -1                   then pos:steps else countTotalSteps (pos:steps, nextGuard guard) grid
    DDown  -> if fst lookaheadPosition >= length grid          then pos:steps else countTotalSteps (pos:steps, nextGuard guard) grid
    DLeft  -> if snd lookaheadPosition <= -1                   then pos:steps else countTotalSteps (pos:steps, nextGuard guard) grid
    DRight -> if snd lookaheadPosition >= (length . head) grid then pos:steps else countTotalSteps (pos:steps, nextGuard guard) grid
    where pos = guardPosition guard
          dir = guardDirection guard
          lookaheadPosition = nextPos pos dir
          nextGuard Guard { guardPosition = pos, guardDirection = dir } =
                    let isNextObstruction = isObstruction lookaheadPosition grid
                        rotateClockwise direction = case direction of
                          DUp -> DRight
                          DRight -> DDown
                          DDown -> DLeft
                          DLeft -> DUp
                        newPosition  = if isNextObstruction then pos else lookaheadPosition
                        newDirection = if isNextObstruction then rotateClockwise dir else dir
                    in  Guard { guardPosition = newPosition, guardDirection = newDirection }

get :: [[a]] -> Position -> a
get grid (y, x) = grid !! y !! x

type RowIndex = Int

findGuardPosition :: [(RowIndex, [Char])] -> Maybe Position
findGuardPosition [] = Nothing
findGuardPosition (row:rows) =
  let result = elemIndex guard $ snd row
  in  case result of
           Just col -> Just (fst row, col)
           Nothing  -> findGuardPosition rows

data Direction = DLeft | DRight | DUp | DDown deriving (Show)

isObstruction :: Position -> [[Char]] -> Bool
isObstruction (row, col) grid = grid !! row !! col == obstruction

nextPos :: Position -> Direction -> Position
nextPos (row, col) direction = case direction of
  DLeft  -> (row, col - 1)
  DRight -> (row, col + 1)
  DUp    -> (row - 1, col)
  DDown  -> (row + 1, col)

