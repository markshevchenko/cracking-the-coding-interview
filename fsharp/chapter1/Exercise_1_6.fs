module Exercise_1_6

open System.Text


let internal appendCharAndCount (stringBuilder: StringBuilder) (c: char) (count: int) =
    stringBuilder.Append(c)
                 .Append(count.ToString())
               |> ignore


let internal rle (s: string) =
    let mutable count = 0
    let mutable prev = '\000'
    let stringBuilder = new StringBuilder()
    
    for c in s do
        if count = 0 then
            prev <- c
            count <- 1
        else if c = prev then
            count <- count + 1
        else
            appendCharAndCount stringBuilder prev count
            prev <- c
            count <- 1
    
    if count > 0 then
        appendCharAndCount stringBuilder prev count
            
    stringBuilder.ToString()


let public pack source =
    let packed = rle source
    if packed.Length < source.Length
    then packed
    else source