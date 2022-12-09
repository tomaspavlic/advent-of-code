use std::{cmp, fs};

fn main() {
    let input_file_content = fs::read_to_string("input/day8.txt").unwrap();

    let trees: Vec<Vec<u8>> = input_file_content
        .lines()
        .map(|line| line.as_bytes().iter().map(|c| c - 48).collect())
        .collect();

    let count1 = (1..trees.len() - 1)
        .map(|y| {
            (1..trees[0].len() - 1)
                .filter(|&x| is_visible(&trees, x, y))
                .count()
        })
        .sum::<usize>();
    println!("1.part: {}", count1 + 2 * (trees.len() - 2 + trees[0].len()));

    let count2 = scenic_score(&trees);
    println!("2.part: {}", count2);
}

fn is_visible(trees: &[Vec<u8>], x: usize, y: usize) -> bool {
    let curr = trees[y][x];
    let vert: Vec<_> = trees.iter().map(|t| t[x]).collect();

    let left = &trees[y][..x];
    let right = &trees[y][x + 1..];
    let top = &vert[..y];
    let bottom = &vert[y + 1..];

    left.iter().all(|&t| t < curr)
        || right.iter().all(|&t| t < curr)
        || top.iter().all(|&t| t < curr)
        || bottom.iter().all(|&t| t < curr)
}

fn count_distance(tree_row: &[u8], max: u8) -> usize {
    let mut result = 0;
    for &v in tree_row {
        result += 1;
        if v >= max {
            break;
        }
    }
    result
}

fn scenic_score(trees: &[Vec<u8>]) -> usize {
    let mut max = 0;
    for y in 1..trees.len() - 1 {
        for x in 1..trees[0].len() - 1 {
            let curr = trees[y][x];
            let vertical: Vec<_> = trees.iter().map(|t| t[x]).collect();
            let left_rev = trees[y][..x].iter().rev().map(|&c| c).collect::<Vec<_>>();
            let left = count_distance(&left_rev, curr);
            let right = count_distance(&trees[y][x + 1..], curr);
            let top_rev = vertical[..y].iter().rev().map(|&c| c).collect::<Vec<_>>();
            let top = count_distance(&top_rev, curr);
            let bottom = count_distance(&vertical[y + 1..], curr);
            max = cmp::max(left * right * top * bottom, max);
        }
    }
    max
}
