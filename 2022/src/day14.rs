use std::{fs, iter};

fn main() {
    let input_file_content = fs::read_to_string("input/day14.txt").unwrap();

    let pos: Vec<Vec<_>> = input_file_content
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pos_str| pos_str.split_once(',').unwrap())
                .map(|pos_str_parts| {
                    (
                        pos_str_parts.0.parse::<i32>().unwrap(),
                        pos_str_parts.1.parse::<i32>().unwrap(),
                    )
                })
                .collect()
        })
        .collect();

    let bottom = pos.iter().flatten().map(|(_x, y)| *y).max().unwrap() as usize;
    let mut cave = vec![vec!['.'; 1000]; 1000];

    for coords in pos {
        for pos in coords.windows(2) {
            let (mut x, mut y) = pos[0];

            if pos[0].0 == pos[1].0 {
                let move_y = pos[1].1 - pos[0].1;
                for _ in 0..=move_y.abs() {
                    cave[y as usize][x as usize] = '#';
                    y = y + move_y.signum();
                }
                continue;
            }

            if pos[0].1 == pos[1].1 {
                let move_x = pos[1].0 - pos[0].0;
                for _ in 0..=move_x.abs() {
                    cave[y as usize][x as usize] = '#';
                    x = x + move_x.signum();
                }
                continue;
            }
        }
    }

    let mut cave_part2 = cave.clone();
    let part1 = iter::repeat_with(|| snow_fall(&mut cave, Direction::Down, 500, 0, true))
        .take_while(|t| *t)
        .count();

    println!("1.part: {}", part1);

    build_bottom(&mut cave_part2, bottom);
    let part2 = iter::repeat_with(|| snow_fall(&mut cave_part2, Direction::Down, 500, 0, false))
        .take_while(|t| *t)
        .count();

    println!("2.part: {}", part2 + 1);
}

enum Direction {
    Down,
    Left,
    Right,
}

fn build_bottom(cave: &mut Vec<Vec<char>>, bottom: usize) {
    for x in 0..cave[bottom + 2].len() {
        cave[bottom + 2][x] = '#';
    }
}

fn snow_fall(cave: &mut Vec<Vec<char>>, dir: Direction, x: i32, y: i32, endless_void: bool) -> bool {
    if endless_void && (y as usize) == cave.len() - 1 {
        return false;
    }
    match dir {
        Direction::Down => {
            let next = cave[(y + 1) as usize][x as usize];
            if next == '.' {
                snow_fall(cave, Direction::Down, x, y + 1, endless_void)
            } else {
                snow_fall(cave, Direction::Left, x, y, endless_void)
            }
        }
        Direction::Left => {
            let next = cave[(y + 1) as usize][(x - 1) as usize];
            if next == '.' {
                snow_fall(cave, Direction::Down, x - 1, y + 1, endless_void)
            } else {
                snow_fall(cave, Direction::Right, x, y, endless_void)
            }
        }
        Direction::Right => {
            let next = cave[(y + 1) as usize][(x + 1) as usize];
            if next == '.' {
                return snow_fall(cave, Direction::Down, x + 1, y + 1, endless_void);
            } else {
                if x == 500 && y == 0 {
                    return false;
                }
                cave[y as usize][x as usize] = 'o';
                return true;
            }
        }
    }
}