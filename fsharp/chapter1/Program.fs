open Exercise_1_1
open Exercise_1_2
open Exercise_1_3
open Exercise_1_4

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
    
    let v = [|'M';  'r';  ' ';  'J';  'o'; 'h'; 'n'; ' '; 'S'; 'm'; 'i'; 't'; 'h'; ' '; ' '; ' '; ' ' |]
    printfn "1.3 Before replaceSpacebars: %A" v
    replaceSpacebars v 13
    printfn "1.3 After prelaceSpacebars: %A" v
    printfn ""
    
    printfn "1.4 isPalindromePermutation \"Tact Coa\" -> %b" (isPalindromePermutation "Tact Coa")
    printfn "1.4 isPalindromePermutation \"fffggh\" -> %b" (isPalindromePermutation "fffggh")
    printfn ""
    
    0