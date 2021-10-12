module Exercise_1_5

let levenshtein (s1: string) (s2: string) =
    let mutable matrix = Array2D.create (s1.Length + 1) (s2.Length + 1) 0
    
    for i = 1 to s1.Length do
        matrix.[i, 0] <- i
        
    for j = 1 to s2.Length do
        matrix.[0, j] <- j

    for i = 1 to s1.Length do
        for j = 1 to s2.Length do
            let insertCost = matrix.[i, j - 1] + 1
            let deleteCost = matrix.[i - 1, j] + 1
            let replaceCost = matrix.[i - 1, j - 1] + if s1.[i - 1] = s2.[j - 1] then 0 else 1
            
            let distance = [insertCost; deleteCost; replaceCost] |> Seq.min
            matrix.[i, j] <- distance
    
    matrix.[s1.Length, s2.Length]
    
let public isLevenshteinDistance0Or1 s1 s2 =
    let distance = levenshtein s1 s2
    
    distance = 0 || distance = 1