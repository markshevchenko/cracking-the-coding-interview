module Exercise_1_7_tests

open System
open Xunit
open Exercise_1_7


[<Fact>]
let ``rotate90 - with 1 2 | 3 4 - return 3 1 | 4 2`` () =
    let image = array2D [
        [ 1; 2; ];
        [ 3; 4; ];
    ]
    
    rotate90 image
    
    let expected = array2D [
        [ 3; 1; ];
        [ 4; 2; ];
    ]
    
    Assert.Equal<int[,]>(expected, image)


[<Fact>]
let ``rotate90 - with 1 2 3 | 4 5 6 | 7 8 9 - return 7 4 1 | 8 5 2 | 9 3 6`` () =
    let image = array2D [
        [ 1; 2; 3; ];
        [ 4; 5; 6; ];
        [ 7; 8; 9; ];
    ]
    
    rotate90 image
    
    let expected = array2D [
        [ 7; 4; 1; ];
        [ 8; 5; 2; ];
        [ 9; 6; 3; ];
    ]
    
    Assert.Equal<int[,]>(expected, image)
    
    
[<Fact>]
let ``rotate90 - with non-square matrix - throws ArgumentException`` () =
    let nonSquareMatrix = array2D [
        [ 1; 2; 3; ];
        [ 4; 5; 6; ];
    ]
    
    Assert.Throws<ArgumentException>(fun () -> rotate90 nonSquareMatrix)