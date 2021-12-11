open System.IO

type Octopus = NotFlashed of byte | Flashed

let adjacent (x, y) = [ (x, y + 1); (x, y - 1); (x - 1, y); (x + 1, y); (x + 1, y + 1); (x - 1, y - 1); (x + 1, y - 1); (x - 1, y + 1) ]
let increment = function NotFlashed v -> NotFlashed (v + 1uy) | Flashed -> Flashed
let increase = Map.map (fun _ -> function NotFlashed v -> NotFlashed (v + 1uy) | Flashed -> NotFlashed 1uy)
let countFlashed = Map.fold (fun c _ -> function Flashed -> c + 1 | _ -> c) 0

let rec flash octopuses = 
    let flashing = octopuses |> Map.filter (fun _ -> function NotFlashed v when v > 9uy -> true | _ -> false)
    let map = Map.fold (fun c key _ -> Map.add key Flashed c) octopuses flashing
    if Map.isEmpty flashing then map 
    else 
        flashing 
        |> Map.keys 
        |> Seq.collect adjacent
        |> Seq.filter (fun k -> Map.containsKey k map) 
        |> Seq.fold (fun m key -> Map.change key (Option.map increment) m) map
        |> flash

let octopuses = 
    File.ReadAllLines("inputs/day11_input.txt") 
    |> Seq.mapi (fun x arr -> Seq.mapi (fun y c -> ((x, y), NotFlashed (byte c - byte '0'))) arr) 
    |> Seq.collect id 
    |> Map.ofSeq

let part1, _ = Seq.fold (fun (count, oct) _ -> let oct' = increase oct |> flash in count + countFlashed oct', oct') (0, octopuses) [ 1 .. 100 ]

let part2, _ = 
    Seq.initInfinite id
    |> Seq.scan (fun oct _ -> oct |> increase |> flash) octopuses
    |> Seq.indexed
    |> Seq.find (snd >> Map.forall (fun _ -> function Flashed -> true | _ -> false))