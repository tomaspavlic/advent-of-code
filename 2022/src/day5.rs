#![feature(let_chains)]

use std::collections::LinkedList;
use std::fs;
use std::iter::Peekable;
use std::vec::Vec;

fn main() {
    let input_file_content = fs::read_to_string("input/day5.txt").unwrap();
    let (mut crate_stacks, moves) = parse_input(&input_file_content);
    let mut crate_stacks2 = crate_stacks.clone();

    for mov in moves.iter() {
        let (from_stack, to_stack) = get_mut_pair(&mut crate_stacks, mov.from - 1, mov.to - 1);
        for _ in 0..mov.count {
            to_stack.push_front(from_stack.pop_front().unwrap());
        }
    }

    println!("1.part: {}", to_string(crate_stacks));

    for mov in moves.iter() {
        let (from_stack, to_stack) = get_mut_pair(&mut crate_stacks2, mov.from - 1, mov.to - 1);
        let mut new_stack = to_stack.split_off(0);
        for _ in 0..mov.count {
            to_stack.push_back(from_stack.pop_front().unwrap());
        }
        to_stack.append(&mut new_stack)
    }

    println!("2.part: {}", to_string(crate_stacks2));
}

fn read_number<I>(chars: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    let mut n = 0;
    while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
        n *= 10;
        n += (c as usize) - 48;
    }
    n
}

fn skip_next<I>(chars: &mut I, c: usize)
where
    I: Iterator,
{
    for _ in 0..c {
        chars.next();
    }
}

struct Movement {
    count: usize,
    from: usize,
    to: usize,
}

impl From<&str> for Movement {
    // I hate regex this should be quicker!
    fn from(s: &str) -> Self {
        let mut iter = s.chars().into_iter().peekable();
        skip_next(&mut iter, 5);
        let count = read_number(&mut iter);
        skip_next(&mut iter, 6);
        let from = read_number(&mut iter);
        skip_next(&mut iter, 4);
        let to = read_number(&mut iter);

        Self { count, from, to }
    }
}

fn get_mut_pair<'a, T>(vec: &'a mut Vec<T>, a: usize, b: usize) -> (&'a mut T, &'a mut T) {
    unsafe {
        let a_item = vec.get_mut(a).unwrap() as *mut _;
        let b_item = vec.get_mut(b).unwrap() as *mut _;

        (&mut *a_item, &mut *b_item)
    }
}

fn parse_input(s: &str) -> (Vec<LinkedList<char>>, Vec<Movement>) {
    let mut input_parts = s.split("\n\n");
    let arrangment = input_parts.next().unwrap();

    let num_of_cargo_stacks = arrangment.lines().next().unwrap().len() / 4;
    let mut crate_stacks = vec![LinkedList::new(); num_of_cargo_stacks + 1];

    for line in arrangment.lines() {
        for (i, stack) in crate_stacks.iter_mut().enumerate() {
            if let Some(&c) = line.as_bytes().get((i * 4) + 1) && c.is_ascii_alphabetic() {
                stack.push_back(c as char)
            }
        }
    }

    let moves: Vec<_> = input_parts
        .next()
        .unwrap()
        .lines()
        .map(Movement::from)
        .collect();

    (crate_stacks, moves)
}

fn to_string(crate_stacks: Vec<LinkedList<char>>) -> String {
    crate_stacks
        .iter()
        .map(|stack| stack.front().unwrap().clone())
        .collect()
}
