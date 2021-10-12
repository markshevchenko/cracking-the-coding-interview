module Exercise_1_2

let public isPermutation (s1: string) (s2: string) =
    let sortedS1 = s1 |> Seq.sort
    let sortedS2 = s2 |> Seq.sort
    
    let compare (a: char) b = a.CompareTo(b)
    
    (Seq.compareWith compare sortedS1 sortedS2) = 0