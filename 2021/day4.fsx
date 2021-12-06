#r "nuget: FParsec"

open FParsec
open System.Text

type Value = uint8 * bool
type Board = Value[,]
type GameState = Play of Board list | Winner of Board * uint8 | Score of int

let devider = newline >>. newline
let number = (pchar ' ' >>. puint8) <|> puint8 |>> fun v -> (v, false)
let board = many (sepBy1 number (skipChar ' ') .>> newline) |>> array2D
let parser = (sepBy puint8 (pchar ',') .>> devider) .>>. (sepBy board newline)

let markValue value = Array2D.map (fun (n, e) -> if n = value then (n, true) else (n, e))

let bingo board = 
    [ 0 .. Array2D.length1 board - 1 ] |> Seq.exists (fun i -> board[*, i] |> Seq.forall snd) ||
    [ 0 .. Array2D.length2 board - 1 ] |> Seq.exists (fun i -> board[i, *] |> Seq.forall snd)

let sumOfAllUnmarkedNumbers (board: Board) = 
    let mutable sum = 0
    Array2D.iter (fun (v, e) -> if not e then sum <- sum + int(v) else ()) board
    sum

let firstWinner boards value =
    let newBoards = boards |> List.map (markValue value)
    newBoards 
    |> Seq.tryFind bingo
    |> Option.map (fun winner -> Winner (winner, value))
    |> Option.defaultValue (Play newBoards)

let lastWinner boards value = 
    let newBoards = boards |> List.map (markValue value)
    if newBoards |> Seq.forall bingo then 
        let winner = boards |> Seq.find (bingo >> not) |> markValue value
        Winner (winner, value)
    else Play newBoards

let gameLoop f gameState value =
    match gameState with
    | Winner (board, w) -> Score (sumOfAllUnmarkedNumbers board * int(w))
    | Play boards -> f boards value
    | _ -> gameState

let score = function Score w -> w | _ -> failwith "Winner not found!"

let (inputs, boards) = 
    runParserOnFile parser () "inputs/day4_input.txt" Encoding.UTF8
    |> function Success (v, _, _) -> v | Failure (err, _, _) -> failwith err
    
inputs |> List.fold (gameLoop firstWinner) (Play boards) |> score |> printfn "PART1: %A" 
inputs |> List.fold (gameLoop lastWinner) (Play boards) |> score |> printfn "PART2: %A"