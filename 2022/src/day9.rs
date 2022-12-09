use std::{collections::HashSet, fs};

fn main() {
    let input_file_content = fs::read_to_string("input/day9.txt").unwrap();

    let moves: Vec<_> = input_file_content
        .lines()
        .map(|line| line.split(' ').collect::<Vec<&str>>())
        .map(|c| (c[0], c[1].parse::<i32>().unwrap()))
        .collect();

    let part1 = part1(&moves);
    println!("1.part: {}", part1);
    
    let part2 = part2(&moves);
    println!("2.part: {}", part2);
}

fn part2(moves: &[(&str, i32)]) -> usize {
    let mut ropes = vec![Position::new(); 10];
    let mut set: HashSet<Position> = HashSet::new();

    for (direction, distance) in moves {
        for _ in 0..*distance {
            let head = ropes.get_mut(0).unwrap();

            match *direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "D" => head.y -= 1,
                "U" => head.y += 1,
                _ => todo!(),
            }

            for i in 0..ropes.len() - 1 {
                let (head, tail) = get_mut_pair(&mut ropes, i, i + 1);
                tail.follow(&head);
                set.insert(ropes.last().unwrap().clone());
            }
        }
    }

    set.len()
}

fn part1(moves: &[(&str, i32)]) -> usize {
    let mut head = Position::new();
    let mut tail = Position::new();
    let mut set: HashSet<Position> = HashSet::new();

    for (direction, distance) in moves {
        for _ in 0..*distance {
            match *direction {
                "R" => head.x += 1,
                "L" => head.x -= 1,
                "D" => head.y -= 1,
                "U" => head.y += 1,
                _ => todo!(),
            }

            tail.follow(&head);
            set.insert(tail.clone());            
        }
    }

    set.len()
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl Position {
    fn new() -> Self {
        Position { x: 0, y: 0 }
    }

    fn follow(&mut self, head: &Self) {
        if head.x.abs_diff(self.x) > 1 || head.y.abs_diff(self.y) > 1 {
            self.x = self.x + (head.x - self.x).signum();
            self.y = self.y + (head.y - self.y).signum();
        }
    } 
}

fn get_mut_pair<'a, T>(vec: &'a mut Vec<T>, a: usize, b: usize) -> (&'a mut T, &'a mut T) {
    unsafe {
        let a_item = vec.get_mut(a).unwrap() as *mut _;
        let b_item = vec.get_mut(b).unwrap() as *mut _;

        (&mut *a_item, &mut *b_item)
    }
}