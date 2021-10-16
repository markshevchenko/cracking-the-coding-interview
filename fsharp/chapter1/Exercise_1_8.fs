module Exercise_1_8

open System.Collections.Generic

let public zeroColumnsRows (matrix: int[,]) =
    let zeroRows = HashSet<int>()
    let zeroColumns = HashSet<int>()
    
    for i = 0 to matrix.GetLength(0) - 1 do
        for j = 0 to matrix.GetLength(1) - 1 do
            if matrix.[i, j] = 0 then
                zeroColumns.Add(j) |> ignore
                zeroRows.Add(i) |> ignore
                
    for row in zeroRows do
        for j = 0 to matrix.GetLength(1) - 1 do
            matrix.[row, j] <- 0
            
    for column in zeroColumns do
        for i = 0 to matrix.GetLength(0) - 1 do
            matrix.[i, column] <- 0
    