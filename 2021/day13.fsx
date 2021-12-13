#r "nuget: FParsec"

open FParsec
open System.Text

type Fold = Up of int | Left of int
let pdots = many (pint32 .>>. (skipChar ',' >>. pint32) .>> skipNewline) |>> Set.ofList
let pfold = pchar 'x' <|> pchar 'y' .>> skipChar '=' .>>. pint32 |>> function ('x', v) -> Left v | ('y', v) -> Up v | _ -> invalidOp "Unknown fold operation"
let pfoldline = skipString "fold along " >>. pfold .>> opt skipNewline
let parser = pdots .>>. (skipNewline >>. many pfoldline)

let page, folds = runParserOnFile parser () "inputs/day13_input.txt" Encoding.UTF8 |> function Success (r, _, _) -> r | _ -> invalidOp "Can not parse"

let printPage p = 
    for y in 0 .. (p |> Seq.map snd |> Seq.max) do
        for x in 0 .. (p |> Seq.map fst |> Seq.max) do 
            if Set.contains (x, y) p then printf "#" else printf "."
        printf "\n"

let foldPage p = 
    function
    | Up y -> Set.map (fun (x, y') -> if y' > y then (x, y - (y' - y)) else (x, y')) p
    | Left x -> Set.map (fun (x', y) -> if x' > x then (x - (x' - x), y) else (x', y)) p

folds |> List.take 1 |> List.fold foldPage page |> Set.count |> printfn "PART1: %d"
folds |> List.fold foldPage page |> printPage