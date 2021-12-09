open System.IO
open System.Collections.Generic

let spread (x, y) = [ (x, y + 1); (x, y - 1); (x - 1, y); (x + 1, y) ]

let tryGet heightmap (x, y) =
    if x >= 0 && x < Array2D.length1 heightmap && y >= 0 && y < Array2D.length2 heightmap 
    then Some heightmap[x, y] 
    else None 

let lowPoint heightmap (x, y) = 
    let value = Array2D.get heightmap x y
    if spread (x, y) |> Seq.choose (tryGet heightmap) |> Seq.forall (fun v -> v > value) then Some (x, y, value) else None

let basin heightmap lowPoint =
    let visited = HashSet()
    let rec inner coord =  
        visited.Add(coord) |> ignore
        spread coord
        |> List.filter (visited.Contains >> not)
        |> List.filter (tryGet heightmap >> Option.map (fun v -> v <> 9uy) >> Option.defaultValue false)
        |> List.iter inner
    inner lowPoint
    visited.Count + 1

let heightmap = File.ReadAllLines("inputs/day9_input.txt") |> Seq.map (Seq.map (fun c -> uint8 c - uint8 '0')) |> array2D

let lowPoints = 
    [ for x in 0 .. Array2D.length1 heightmap - 1 do
        for y in 0 .. Array2D.length2 heightmap - 1 do x, y ] 
    |> List.choose (lowPoint heightmap)

let part1 = lowPoints |> List.map (fun (_, _, v) -> int v + 1) |> List.reduce (+)
let part2 = lowPoints |> List.map (fun (x, y, _) -> basin heightmap (x, y)) |> List.sortDescending |> List.take 3 |> List.reduce (*)