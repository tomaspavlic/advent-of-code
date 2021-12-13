open System.IO
open System

type Cave = Start | Small of string | Big of string | End

module Cave =
    let isEnd = function End -> true | _ -> false
    let isSmall = function Small _ -> true | _ -> false
    let from = function "start" -> Start | "end" -> End | _ as x when Char.IsUpper(x[0]) -> Big x | _ as x -> Small x

let add (item: 'a) = (function Some list -> item :: list | None -> [ item ]) >> Some

let anyTwice = List.filter (function Small _ -> true | _ -> false) >> List.groupBy id  >> List.map (snd >> List.length) >> List.contains 2

let countPaths stopPredicate (adjMap: Map<Cave, Cave list>) =
    let rec inner visited (c: Cave) =
        if Cave.isEnd c then 1
        elif stopPredicate c visited then 0
        else
            let adjenced = adjMap[c] |> List.filter (function Start -> false | _ -> true )
            Seq.fold (fun s adj -> s + inner (c :: visited) adj) 0 adjenced

    inner [] Start

let adjacencyCaveMap = 
    File.ReadAllLines("inputs/day12_input.txt") 
    |> Seq.map (fun line -> let p = line.Split('-') in Cave.from p[0], Cave.from p[1])
    |> Seq.fold (fun m (c1, c2) -> Map.change c1 (add c2) m |> Map.change c2 (add c1)) Map.empty

let stop1 cave visited = Cave.isSmall cave && List.contains cave visited
let stop2 cave visited = Cave.isSmall cave && List.contains cave visited && anyTwice visited

countPaths stop1 adjacencyCaveMap |> printfn "PART1: %d"
countPaths stop2 adjacencyCaveMap |> printfn "PART2: %d"