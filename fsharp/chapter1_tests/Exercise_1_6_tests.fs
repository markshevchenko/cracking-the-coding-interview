module Exercise_1_6_tests

open System.Text
open Xunit
open Exercise_1_6


[<Fact>]
let ``appendCharAndCount - with empty string a 3 - returns a3`` () =
    let stringBuilder = StringBuilder()
    
    appendCharAndCount stringBuilder 'a' 3
    
    Assert.Equal("a3", stringBuilder.ToString())


[<Fact>]
let ``rle - with empty string - returns empty string`` () =
    Assert.Equal("", rle "")


[<Fact>]
let ``rle - with aabcccccaaa - returns a2b1c5a3`` () =
    Assert.Equal("a2b1c5a3", rle "aabcccccaaa")


[<Fact>]
let ``pack - with abc - returns abc`` () =
    Assert.Equal("abc", pack "abc")
    
    
[<Fact>]
let ``pack - with aabcccccaaa - returns a2b1c5a3`` () =
    Assert.Equal("a2b1c5a3", pack "aabcccccaaa")
