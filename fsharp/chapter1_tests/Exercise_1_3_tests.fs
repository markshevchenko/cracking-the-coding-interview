module Exercise_1_3_tests

open Xunit
open Exercise_1_3


[<Fact>]
let ``replaceSpacebars - with Mr John Smith 13 - returns Mr%20John%20Smith`` () =
    let array = [|'M'; 'r'; ' '; 'J'; 'o'; 'h'; 'n'; ' '; 'S'; 'm'; 'i'; 't'; 'h'; ' '; ' '; ' '; ' ' |]
    
    replaceSpacebars array 13
    
    let expected = [|'M'; 'r'; '%'; '2'; '0'; 'J'; 'o'; 'h'; 'n'; '%'; '2'; '0'; 'S'; 'm'; 'i'; 't'; 'h'|]
    
    Assert.Equal<char seq>(expected, array)