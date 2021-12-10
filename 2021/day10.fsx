open System.IO

let inputs = File.ReadAllLines("inputs/day10_input.txt") |> Seq.map Seq.toList |> Seq.toList
let bMap = [ ('(', ')'); ('[', ']'); ('{', '}'); ('<', '>') ] |> Map.ofList
let isOpeningBracket c = Map.containsKey c bMap
let isMatchingClosingBracket c = function head::_ -> bMap[head] = c | _ -> false

let incorrectScore = function ')' -> 3 | ']' -> 57 | '}' -> 1197 | '>' -> 25137 | _ -> 0
let completionValue = function '(' -> 1L | '[' -> 2L | '{' -> 3L | '<' -> 4L | _ -> 0L
let completionScore = Seq.fold (fun s v -> (s * 5L) + completionValue v) 0L

type SyntaxResult = 
    | Incorrect of char 
    | Incomplete of char list

let rec scoring seen = 
    function
    | [] -> Incomplete seen
    | head::tail when isOpeningBracket head -> scoring (head :: seen) tail
    | head::tail when isMatchingClosingBracket head seen -> scoring (List.tail seen) tail
    | head::_ -> Incorrect head

let scorings = inputs |> List.map (scoring [])
let part1 = scorings |> Seq.choose (function Incorrect c -> Some (incorrectScore c) | _ -> None) |> Seq.reduce (+)
let part2 = scorings |> List.choose (function Incomplete rest -> Some (completionScore rest) | _ -> None) |> List.sort
part2[part2.Length / 2]