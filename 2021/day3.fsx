open System.IO
open System

type Operator = Most | Least

let frequentBit operator (lines: string seq) index =
    let counts = lines |> Seq.map (fun line -> line[index]) |> Seq.countBy id |> Map.ofSeq
    match operator with
    | Most when counts['1'] < counts['0'] -> '0'
    | Least when counts['1'] < counts['0'] -> '1'
    | Most -> '1'
    | Least -> '0'

let rating operator lines =
    let rec inner index lines =
        match lines with
        | head :: [] -> head
        | _ ->
            let m = frequentBit operator lines index
            let filtered = lines |> List.filter (fun line -> line[index] = m)
            inner (index + 1) filtered

    inner 0 lines

let lines = File.ReadAllLines "inputs/day3_input.txt"

let gamma = [ 0 .. lines[0].Length - 1 ] |> List.map (frequentBit Most lines) |> List.toArray
let epsilon = [ 0 .. lines[0].Length - 1 ] |> List.map (frequentBit Least lines) |> List.toArray
let part1 = Convert.ToInt32(new String(gamma), 2) * Convert.ToInt32(new String (epsilon), 2)

let oxygen = lines |> Seq.toList |> rating Most
let co2 = lines |> Seq.toList |> rating Least
let part2 = Convert.ToInt32(oxygen, 2) * Convert.ToInt32(co2, 2)

printfn "PARTI: %d" part1
printfn "PART2: %d" part2