import           System.IO

data State
  = S
  | Q1
  | Q2
  | Q3
  deriving (Show)

main :: IO ()
main = do
  file <- readFile "input.txt"
  let fileLines = words file
  parse fileLines

parse :: [String] -> IO ()
parse (item : xs) = do
  putStr $ item ++ ": "
  case validate S item of
    True  -> putStrLn "Valid!"
    False -> putStrLn "Invalid!"
  parse xs
parse [] = return ()

validate :: State -> String -> Bool
validate S  ('0' : xs) = validate S xs
validate S  ('1' : xs) = validate Q1 xs
validate S  _          = False
validate Q1 ('0' : xs) = validate Q2 xs
validate Q1 _          = False
validate Q2 ('0' : xs) = validate Q3 xs
validate Q2 _          = False
validate Q3 ('0' : xs) = validate Q2 xs
validate Q3 ('1' : xs) = False
validate Q3 []         = True
validate _  _          = False

