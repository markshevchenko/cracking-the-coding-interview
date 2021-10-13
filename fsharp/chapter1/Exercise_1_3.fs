module Exercise_1_3


let public replaceSpacebars (s: char array) length =
    let lastIndex = length - 1
    let mutable currentLastIndex = lastIndex
    
    for i = lastIndex downto 0 do
        if s.[i] = ' '
        then
            for j = currentLastIndex downto (i + 1) do
                s.[j + 2] <- s.[j]
                
            s.[i] <- '%'
            s.[i + 1] <- '2'
            s.[i + 2] <- '0'
            
            currentLastIndex <- currentLastIndex + 2

