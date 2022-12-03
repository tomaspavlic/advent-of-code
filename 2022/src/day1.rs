use std::error::Error;
use std::fs;
use std::result::Result;
use std::vec::Vec;

fn main() -> Result<(), Box<dyn Error>> {
    let input_file_content = fs::read_to_string("input/day1.txt")?;

    let mut calories: Vec<_> = input_file_content
        .split("\n\n")
        .map(|c| c.lines().filter_map(|n| n.trim().parse::<i32>().ok()).sum::<i32>())
        .collect();

    let max = calories.iter().max().ok_or("max() failes")?;
    println!("1.part: {}", max);

    calories.sort_by(|a, b| b.cmp(a));
    let top_three_sum: i32 = calories.iter().take(3).sum();
    println!("2.part: {}", top_three_sum);
    
    Ok(())
}