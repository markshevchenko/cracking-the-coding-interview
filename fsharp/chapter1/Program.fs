open System.Runtime.CompilerServices
open Exercise_1_1
open Exercise_1_2
open Exercise_1_3
open Exercise_1_4
open Exercise_1_5
open Exercise_1_6
open Exercise_1_7
open Exercise_1_8
open Exercise_1_9


[<assembly:InternalsVisibleTo("chapter1_tests")>]
do ()


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
    
    printfn "1.5 isLevenshteinDistance0Or1 \"pale\" \"ple\" -> %b" (isLevenshteinDistance0Or1 "pale" "ple")
    printfn "1.5 isLevenshteinDistance0Or1 \"palex\" \"pale\" -> %b" (isLevenshteinDistance0Or1 "palex" "pale")
    printfn "1.5 isLevenshteinDistance0Or1 \"pale\" \"bale\" -> %b" (isLevenshteinDistance0Or1 "pale" "bale")
    printfn "1.5 isLevenshteinDistance0Or1 \"pale\" \"bake\" -> %b" (isLevenshteinDistance0Or1 "pale" "bake")
    printfn ""
    
    printfn "1.6 pack \"abc\" -> %s" (pack "abc")
    printfn "1.6 pack \"aabcccccaaa\" -> %s" (pack "aabcccccaaa")
    printfn ""
    
    let matrix2 = array2D [
        [1; 2]
        [3; 4]
    ]
    printfn "1.7 Before rotate90:"
    printfn "%A" matrix2
    rotate90 matrix2
    printfn "1.7 After rotate90:"
    printfn "%A" matrix2
    printfn ""
    
    
    let zeroMatrix = array2D [
        [11; 12; 13; 14; 15; 16]
        [21;  0; 23; 24; 25; 26]
        [31; 32; 33; 34; 35; 36]
        [41; 42; 43; 44;  0; 46]
        [51; 52; 53; 54; 55; 56]
    ]
    printfn "1.8 Before zeroColumnsRows:"
    printfn "%A" zeroMatrix
    zeroColumnsRows zeroMatrix
    printfn "1.8 After zeroColumnsRows:"
    printfn "%A" zeroMatrix
    printfn ""
    
    
    printfn "1.9 isRotation \"erbottlewat\" \"waterbottle\" -> %b" (isRotation "erbottlewat" "waterbottle")
    printfn ""
        
    0