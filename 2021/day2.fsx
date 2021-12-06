#r "nuget: FParsec"

open System
open FParsec

let penum<'a> =
    let enumType = typeof<'a>
    Enum.GetNames(enumType)
    |> Seq.map pstringCI
    |> Seq.reduce (<|>)
    |>> fun enumString -> Enum.Parse(enumType, enumString, true) :?> 'a

type Operation = Forward = 0 | Down = 1 | Up = 2
type Position = { horizontal: int; depth: int; aim: int }
let parser =
    sepBy (penum<Operation> .>>. (pchar ' ' >>. pint32)) newline

let parseInput filePath =
    runParserOnFile parser () filePath Text.Encoding.UTF8
    |> function
        | Success (result, _, _) -> result
        | Failure (err, _, _) -> failwithf "Could not parse input filel %s" err

let move pos (op, value) =
    match op with
    | Operation.Forward -> { pos with horizontal = pos.horizontal + value }
    | Operation.Down -> { pos with depth = pos.depth + value }
    | Operation.Up -> { pos with depth = pos.depth - value }
    | _ -> failwith "Unsupported operation!"

let moveWithAim pos (op, value) =
    match op with
    | Operation.Forward -> { pos with horizontal = pos.horizontal + value; depth = pos.depth + (pos.aim * value)}
    | Operation.Down -> { pos with aim = pos.aim + value }
    | Operation.Up -> { pos with aim = pos.aim - value }
    | _ -> failwith "Unsupported operation!"

let moves = parseInput "inputs/day2_input.txt"
let defaultPosition = { horizontal = 0; depth= 0; aim = 0 }
let position = moves |> Seq.fold move defaultPosition
let part1 = position.horizontal * position. depth
let position2 = moves |> Seq. fold moveWithAim defaultPosition 
let part2 = position2.horizontal * position2.depth 
printfn "PART1: %d" part1
printfn "PART2: %d" part2