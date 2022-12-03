use std::collections::HashSet;
use std::error::Error;
use std::fs;
use std::result::Result;

fn priority(c: char) -> u8 {
    let ascii_code = c as u8;
    if ascii_code > 90 {
        ascii_code - 96
    } else {
        ascii_code - 38
    }
}

fn find_priority(group: Vec<HashSet<char>>) -> u32 {
    let mut iter = group.into_iter();
    let intersection = iter
        .next()
        .map(|set| iter.fold(set, |set1, set2| &set1 & &set2))
        .unwrap();

    let common_char = intersection.iter().next().unwrap();

    priority(common_char.clone()) as u32
}

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_content = fs::read_to_string("input/day3.txt")?;

    let sum_part1: u32 = input_file_content
        .lines()
        .map(|line| {
            let chars: Vec<_> = line.chars().collect();
            let (left, right) = chars.split_at(chars.len() / 2);
            let group = vec![
                HashSet::from_iter(Vec::from(left)),
                HashSet::from_iter(Vec::from(right)),
            ];

            find_priority(group)
        })
        .sum();

    println!("1.part: {}", sum_part1);

    let sum_part2: u32 = input_file_content
        .lines()
        .map(|line| HashSet::from_iter(line.chars()))
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|sets| find_priority(sets.to_vec()))
        .sum();

    println!("2.part: {}", sum_part2);

    Ok(())
}
