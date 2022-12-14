use pathfinding::prelude::dijkstra;
use std::fs;

fn main() {
    let input_file_content = fs::read_to_string("input/day12.txt").unwrap();

    let mut heat_map: Vec<_> = input_file_content
        .lines()
        .map(|line| Vec::from(line.as_bytes()))
        .collect();

    let end_pos = find(&heat_map, 'E');
    let start_pos = find(&heat_map, 'S');
    *&mut heat_map[end_pos.1][end_pos.0] = 'z' as u8;
    *&mut heat_map[start_pos.1][start_pos.0] = 'a' as u8;

    let heat_map2 = heat_map.clone();
    let mut distances = vec![];

    for y in 0..heat_map2.len() {
        for x in 0..heat_map2[0].len() {
            if heat_map2[y][x] == 'a' as u8 {
                if let Some((_, distance)) =
                    dijkstra(&(x, y), |p| adjenc(&heat_map, p), |p| *p == end_pos)
                {
                    distances.push(distance);
                }
            }
        }
    }

    distances.sort();
    println!("2.part: {}", distances.first().unwrap());

    let part1 = dijkstra(&start_pos, |p| adjenc(&heat_map, p), |p| *p == end_pos).unwrap();
    println!("1.part: {:?}", part1.1);
}

fn adjenc(heat_map: &Vec<Vec<u8>>, p: &(usize, usize)) -> Vec<((usize, usize), usize)> {
    let (x, y) = *p;
    let current = heat_map[y][x];
    let mut adj = vec![];
    if x > 0 && heat_map[y][x - 1] <= current + 1 {
        adj.push((x - 1, y))
    }
    if x + 1 < heat_map[0].len() && heat_map[y][x + 1] <= current + 1 {
        adj.push((x + 1, y));
    }
    if y + 1 < heat_map.len() && heat_map[y + 1][x] <= current + 1 {
        adj.push((x, y + 1));
    }
    if y > 0 && heat_map[y - 1][x] <= current + 1 {
        adj.push((x, y - 1));
    }
    adj.into_iter().map(|p| (p, 1)).collect()
}

fn find(heat_map: &[Vec<u8>], u: char) -> (usize, usize) {
    for y in 0..heat_map.len() {
        for x in 0..heat_map[0].len() {
            if heat_map[y][x] == u as u8 {
                return (x, y);
            }
        }
    }

    panic!("could not find");
}
