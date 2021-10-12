module Exercise_1_1
    let public containsDuplicates s =
        s |> Seq.sort
          |> Seq.pairwise
          |> Seq.exists (fun pair -> fst pair = snd pair)

