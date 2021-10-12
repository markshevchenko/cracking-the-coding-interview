open System
open Exercise_1_1
open Exercise_1_2

// 1.1
[<EntryPoint>]
let main argv =
    printfn "1.1 containsDuplicates \"\" -> %b" (containsDuplicates "")
    printfn "1.1 containsDuplicates \"abcdef\" -> %b" (containsDuplicates "abcdef")
    printfn "1.1 containsDuplicates \"foo\" -> %b" (containsDuplicates "foo")
    printfn ""
    
    printfn "1.2 isPermutation \"\" \"\" -> %b" (isPermutation "" "")
    printfn "1.2 isPermutation \"abc\" \"bac\" -> %b" (isPermutation "abc" "bac")
    printfn "1.2 isPermutation \"bar\" \"baz\" -> %b" (isPermutation "bar" "baz")
    printfn ""
    
    0