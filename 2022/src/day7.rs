use std::{collections::HashMap, fs};

fn main() {
    let input_file_content = fs::read_to_string("input/day7.txt").unwrap();

    let mut folder_path: Vec<&str> = Vec::new();
    let mut folder_sizes: HashMap<String, u64> = HashMap::new();

    for l in input_file_content.lines() {
        let commands: Vec<&str> = l.split(' ').collect();

        if commands[0] == "$" {
            if commands[1] == "cd" {
                if commands[2] == ".." {
                    folder_path.pop();
                } else {
                    folder_path.push(commands[2]);
                }
            }
        } else {
            if commands[0] != "dir" {
                let file_size: u64 = commands[0].parse().unwrap();
                let mut current_path = String::from("");
                for &folder in folder_path.iter() {
                    if current_path != "/" && folder != "/" {
                        current_path.push('/')
                    }
                    current_path.push_str(folder);
                    folder_sizes
                        .entry(current_path.clone())
                        .and_modify(|v| *v += file_size)
                        .or_insert(file_size);
                }
            }
        }
    }

    let mut values: Vec<u64> = folder_sizes.iter().map(|(_, &v)| v).collect();

    let part1: u64 = values.iter()
        .filter(|&&v| v < 100_000)
        .sum();

    println!("1.part: {}", part1);

    let space_needed = 30_000_000 - (70_000_000 - folder_sizes.get("/").unwrap());
    values.sort();
    let part2 = values.iter().find(|&&v| v > space_needed).unwrap();

    println!("2.part: {}", part2);
}