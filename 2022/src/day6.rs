use std::collections::HashSet;
use std::fs;

fn main() {
    let input_file_content = fs::read_to_string("input/day6.txt").unwrap();
    
    let part1 = first_marker_pos(&input_file_content, 4);
    println!("1.part: {}", part1);

    let part2 = first_marker_pos(&input_file_content, 14);
    println!("2.part: {}", part2);
}

fn first_marker_pos(str: &str, n: usize) -> usize {
    let mut set = HashSet::new();
    for (i, c) in str.chars().enumerate() {
        if set.len() == n {
            return i;
        }
        if set.contains(&c) {
            set.clear();
        }
        set.insert(c);
    }

    panic!("marker not found")
}