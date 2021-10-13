module Exercise_1_7

open System


let public rotate90 (image: int[,]) =
    if image.GetLength(0) <> image.GetLength(1) then
        ArgumentException("Matrix is not square") |> raise
        
    let n = image.GetLength(0)
    
    for i = 0 to n/2 - 1 do
        for j = 0 to (n + 1)/2 - 1 do
            let t = image.[i, j]
            image.[i, j] <- image.[n - 1 - j, i]
            image.[n - 1 - j, i] <- image.[n - 1 - i, n - 1 - j]
            image.[n - 1 - i, n - 1 - j] <- image.[j, n - 1 - i]
            image.[j, n - 1 - i] <- t
    