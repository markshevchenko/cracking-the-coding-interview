module Exercise_1_8_tests

open Xunit
open Exercise_1_8


[<Fact>]
let ``zeroColumnsRows - without zeros - returns same matrix`` () =
    let matrix = array2D [
        [11; 12; 13; 14; 15; 16];
        [21; 22; 23; 24; 25; 26];
        [31; 32; 33; 34; 35; 36];
        [41; 42; 43; 44; 45; 46];
    ]
    
    let expected = array2D [
        [11; 12; 13; 14; 15; 16];
        [21; 22; 23; 24; 25; 26];
        [31; 32; 33; 34; 35; 36];
        [41; 42; 43; 44; 45; 46];
    ]
    
    zeroColumnsRows matrix
    
    Assert.Equal<int[,]>(expected, matrix)
    
    
[<Fact>]
let ``zeroColumnsRows - with zero - sets zeros in column and row`` () =
    let matrix = array2D [
        [11; 12; 13; 14; 15; 16];
        [21; 22; 23;  0; 25; 26];
        [31; 32; 33; 34; 35; 36];
        [41; 42; 43; 44; 45; 46];
    ]
    
    let expected = array2D [
        [11; 12; 13;  0; 15; 16];
        [ 0;  0;  0;  0;  0;  0];
        [31; 32; 33;  0; 35; 36];
        [41; 42; 43;  0; 45; 46];
    ]
    
    zeroColumnsRows matrix
    
    Assert.Equal<int[,]>(expected, matrix)