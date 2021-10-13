module Exercise_1_5


let internal makeMatrix height width =
    let matrix = Array2D.create height width 0

    for i = 1 to height - 1 do
        matrix.[i, 0] <- i
        
    for j = 1 to width - 1 do
        matrix.[0, j] <- j
        
    matrix
    
    
let internal min3 a b c =
    if a < b
    then if a < c
         then a
         else c
    else if b < c
         then b
         else c
         
         
let internal estimateDistance c1 c2 =
    if c1 = c2
    then 0
    else 1


let internal levenshtein (s1: string) (s2: string) =
    let matrix = makeMatrix (s1.Length + 1) (s2.Length + 1)
    
    for i = 1 to s1.Length do
        for j = 1 to s2.Length do
            let insertCost = matrix.[i, j - 1] + 1
            let deleteCost = matrix.[i - 1, j] + 1
            let replaceCost = matrix.[i - 1, j - 1] + (estimateDistance s1.[i - 1] s2.[j - 1])
            
            let distance = min3 insertCost deleteCost replaceCost
            matrix.[i, j] <- distance
    
    matrix.[s1.Length, s2.Length]
    

let public isLevenshteinDistance0Or1 s1 s2 =
    let distance = levenshtein s1 s2
    
    distance = 0 || distance = 1