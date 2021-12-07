open System.IO

let inputs = File.ReadAllText("inputs/day7_input.txt").Split(',') |> Seq.map int |> Seq.toArray

let incremental distance = distance * (distance + 1) / 2
let distances f inputs =
    [ Seq.min inputs .. Seq.max inputs ]
    |> Seq.map (fun c -> inputs |> Seq.map (fun input -> c - input |> abs |> f))

let cheapest = Seq.map (Seq.reduce (+)) >> Seq.min

inputs |> distances id |> cheapest |> printfn "PART1: %d"
inputs |> distances incremental |> cheapest |> printfn "PART2: %d"