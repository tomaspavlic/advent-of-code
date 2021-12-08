#r "nuget: FParsec"

open System.IO
open System

let overlap (encoding: Map<int, string>) digits length overlapCount =
    (encoding[length] |> Seq.filter (fun c -> digits |> Seq.contains c) |> Seq.length) = overlapCount

let decode (encoding: Map<int, string>) (output: string) =
    let overlap' = overlap encoding output
    match output.Length with
    | 6 when overlap' 4 4 -> '9'
    | 6 when overlap' 2 2 -> '0' | 6 -> '6'
    | 5 when overlap' 2 2 -> '3' 
    | 5 when overlap' 4 2 -> '2' | 5 -> '5'
    | 2 -> '1' | 3 -> '7' | 4 -> '4' | _ -> '8'

let filterUnique = Seq.filter (fun d -> [ 2; 4; 7; 3 ] |> Seq.contains (Seq.length d))

let inputs = 
    File.ReadAllLines("inputs/day8_input.txt")
    |> Array.map (fun line -> 
        let i = line.Split(' ')
        let pipeIndex = i |> Array.findIndex(fun x -> x = "|")
        i[..pipeIndex - 1], i[pipeIndex + 1..])

let part1 = inputs |> Seq.map snd |> Seq.map (filterUnique >> Seq.length) |> Seq.reduce (+)

let part2 =
    inputs
    |> Seq.map (fun (inputs, outputs) ->
        let encoding = filterUnique inputs |> Seq.map (fun i -> i.Length, i) |> Map.ofSeq
        let output = outputs |> Seq.map (decode encoding) |> Seq.toArray
        new string(output) |> Convert.ToInt32)
    |> Seq.reduce (+)