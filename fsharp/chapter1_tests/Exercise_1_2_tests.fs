module Exercise_1_2_tests

open Xunit
open Exercise_1_2


[<Fact>]
let ``isPermutation - with empty strings - returns true`` () =
    Assert.True(isPermutation "" "")
    
    
[<Fact>]
let ``isPermutation - with abc bac - return true`` () =
    Assert.True(isPermutation "abc" "bac")


[<Fact>]
let ``isPermutation - with baz baz - return false`` () =
    Assert.False(isPermutation "bar" "baz")
                