﻿open System
open Exercise_1_1

// 1.1
[<EntryPoint>]
let main argv =
    printfn "1.1 containsDuplicates \"\" -> %b" (containsDuplicates "")
    printfn "1.1 containsDuplicates \"abcdef\" -> %b" (containsDuplicates "abcdef")
    printfn "1.1 containsDuplicates \"foo\" -> %b" (containsDuplicates "foo")
    printfn ""
    
    0