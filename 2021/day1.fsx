open System.IO

let measurements =
    File.ReadAllLines("inputs/day1_input.txt")
    |> Seq.map int

let increases input = // seq‹'a» -› int
    input
    |> Seq.pairwise
    |> Seq.filter (fun (a, b) -> a < b)
    |> Seq.length

let part1 = increases measurements 
let part2 = 
    measurements
    |> Seq.windowed 3
    |> Seq.map (Seq.reduce (+))
    |> increases

printfn "PART1: %d" part1
printfn "PART2: %d" part2