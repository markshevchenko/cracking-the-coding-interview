module Exercise_1_4_tests

open Xunit
open Exercise_1_4


[<Fact>]
let ``isPalindromePermutation - with empty string - return true`` () =
    Assert.True(isPalindromePermutation "")
    

[<Fact>]
let ``isPalindromePermutation - with Tact Coa - return true`` () =
    Assert.True(isPalindromePermutation "Tact Coa")


[<Fact>]
let ``isPalindromePermutation - with foo bar baz - return false`` () =
    Assert.False(isPalindromePermutation "foo bar baz")
