import System.IO

main :: IO ()
main = do
    file <- readFile "input.txt"
    let fileLines = lines file
    print fileLines
