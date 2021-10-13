module Exercise_1_5_tests

open Xunit
open Exercise_1_5


[<Fact>]
let ``makeMatrix - with 3 4 - returns 0123 1000 2000`` () =
    let matrix = makeMatrix 3 4
    
    let expected = array2D [
        [0; 1; 2; 3];
        [1; 0; 0; 0];
        [2; 0; 0; 0];
    ]
    
    Assert.Equal<int[,]>(expected, matrix)
    
    
[<Theory>]
[<InlineData(1, 2, 3, 1)>]
[<InlineData(1, 3, 2, 1)>]
[<InlineData(2, 1, 3, 1)>]
[<InlineData(2, 3, 1, 1)>]
[<InlineData(3, 1, 2, 1)>]
[<InlineData(3, 2, 1, 1)>]
let ``min3 - with 3 numbers - returns min`` (a, b, c, expected) =
    Assert.Equal(expected, min3 a b c)
    
    
[<Theory>]
[<InlineData('a', 'a', 0)>]
[<InlineData('a', 'b', 1)>]
let ``estimateDistance - with 2 chars - returns distance`` (c1, c2, expected) =
    Assert.Equal(expected, estimateDistance c1 c2)


[<Fact>]
let ``levenshtein - with bar baz - returns 1`` () =
    let actual = levenshtein "bar" "baz"
    
    Assert.Equal(1, actual)


[<Fact>]
let ``levenshtein - with foo bar - returns 3`` () =
    let actual = levenshtein "foo" "bar"
    
    Assert.Equal(3, actual)