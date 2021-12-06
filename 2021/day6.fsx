open System.IO

let empty = [ 0 .. 8 ] |> Seq.map (fun day -> day, 0L) |> Map.ofSeq
let init =
    File.ReadAllText("inputs/day6_input.txt").Split(',')
    |> Seq.map int
    |> Seq.fold (fun m day -> Map.change day (function Some count -> Some (count + 1L) | _ -> None) m) empty

let decrease fish =
    fish
    |> Map.map (fun day _ ->
        match day with
        | 6 -> fish[0] + fish[day + 1]
        | 8 -> 0L
        | _ -> fish[day + 1])

let spawn fish =
    decrease fish
    |> Map.change 8 (function Some _ -> Some fish[0] | _ -> None)

let simulate init days =
    [ 2 .. days ]
    |> List.fold (fun m _ -> spawn m) (decrease init)
    |> Map.values
    |> Seq.reduce (+)

simulate init 80 |> printfn "PART1: %d"
simulate init 256 |> printfn "PART2: %d"