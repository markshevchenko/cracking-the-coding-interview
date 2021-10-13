module Exercise_1_1_tests

open Xunit
open Exercise_1_1


[<Fact>]
let ``containsDuplicates - with empty string - returns false`` () =
    Assert.False(containsDuplicates "")
    

[<Fact>]
let ``containsDuplicates - with no duplicates - returns false`` () =
    Assert.False(containsDuplicates "abcdef")
    
  
[<Fact>]
let ``containsDuplicates - with repeated letter - returns true`` () =
    Assert.True(containsDuplicates "foo")
    
