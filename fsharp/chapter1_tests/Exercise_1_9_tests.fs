module Exercise_1_9_tests

open Xunit
open Exercise_1_9


[<Fact>]
let ``isRotation - with empty strings - returns true`` () =
    Assert.True(isRotation "" "")
    
    
[<Fact>]
let ``isRotation - with waterbottle and erbottlewat - return true`` () =
    Assert.True(isRotation "erbottlewat" "waterbottle")