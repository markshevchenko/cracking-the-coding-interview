module Exercise_1_4

open System

let public isPalindromePermutation (s: string) =
    let letters = s
               |> Seq.filter (not << Char.IsWhiteSpace)
               |> Seq.map Char.ToLower
    
    let letterCounts = letters
                    |> Seq.groupBy id
                    |> Seq.map (fun (c, group) -> (c, Seq.length group))
                         
    let evenLetterCounts = letterCounts
                        |> Seq.filter (fun (_, count) -> count % 2 = 0)
    
    let oddCharCount = (letterCounts |> Seq.length) - (evenLetterCounts |> Seq.length)
    
    oddCharCount = 0 || oddCharCount = 1
