#r "nuget: FParsec"

open FParsec
open System.Text

let pcoord = pint32 .>>. (pchar ',' >>. pint32)
let parser = sepBy (pcoord .>>. (pstring " -> " >>. pcoord)) newline

let inputs =
    runParserOnFile parser () "inputs/day5_input.txt" Encoding.UTF8
    |> function Success (v, _, _) -> v | Failure (err, _, _) -> failwith err

let coordinates diagonal ((x1, y1), (x2, y2)) = 
    if x1 = x2 then [ for i in min y1 y2 .. max y1 y2 -> x1, i ]
    elif y1 = y2 then [ for i in min x1 x2 .. max x1 x2 -> i, y1 ]
    elif diagonal then 
        [ for i in 0 .. abs (y1 - y2) do
            let x = if x1 - x2 < 0 then x1 + i else x1 - i
            let y = if y1 - y2 < 0 then y1 + i else y1 - i
            x, y ]
    else []

let countOverlap lines = 
    lines
    |> List.fold (fun map c -> Map.change c (function None -> Some 1 | Some v -> Some (v + 1)) map) Map.empty
    |> Map.filter (fun _ v -> v > 1)
    |> Map.count

let lines1 = inputs |> List.collect (coordinates false)
let lines2 = inputs |> List.collect (coordinates true)

countOverlap lines1 |> printfn "PART1: %d"
countOverlap lines2 |> printfn "PART2: %d"