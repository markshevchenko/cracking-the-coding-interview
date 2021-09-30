open System

// 1.1
let containsDublicates s =
    s |> Seq.sort
      |> Seq.pairwise
      |> Seq.exists (fun pair -> fst pair = snd pair)

[<EntryPoint>]
let main argv =
    // 1.1
    assert not (containsDublicates "")
    assert not (containsDublicates "abcdef")
    assert containsDublicates "foo"
    
    0