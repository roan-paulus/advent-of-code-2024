main = do
  contents <- getContents
  mapM print contents
