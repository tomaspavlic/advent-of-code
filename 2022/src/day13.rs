use std::{cell::RefCell, cmp::Ordering, fs, iter::Peekable, rc::Rc};

mod helpers;
use helpers::{Cell, Linkable, Node};

fn main() {
    let mut input_file_content = fs::read_to_string("input/day13.txt").unwrap();
    let good_pairs: usize = input_file_content
        .split("\n\n")
        .map(|lines| lines.split_once('\n').unwrap())
        .enumerate()
        .map(|(i, (left, right))| {
            if let Ordering::Greater = compare(line_to_tree(left), line_to_tree(right)) {
                return i + 1;
            }
            return 0;
        })
        .sum();

    println!("1.part: {}", good_pairs);

    input_file_content.push_str("\n[[2]]");
    input_file_content.push_str("\n[[6]]");
    let mut trees: Vec<_> = input_file_content
        .replace("\n\n", "\n")
        .lines()
        .map(|l| line_to_tree(l))
        .collect();

    trees.sort_by(|a, b| compare(b.clone(), a.clone()));

    let ts: Vec<_> = trees
        .iter()
        .map(|node| {
            let mut s = String::new();
            to_string(node, &mut s);
            s
        })
        .collect();

    let p1 = ts
        .iter()
        .enumerate()
        .find(|(_, s)| *s == "[[2]]")
        .unwrap()
        .0
        + 1;

    let p2 = ts
        .iter()
        .enumerate()
        .find(|(_, s)| *s == "[[6]]")
        .unwrap()
        .0
        + 1;

    println!("2.part: {}", p1 * p2);
}

fn to_string(node: &Rc<RefCell<Node<usize>>>, s: &mut String) {
    match node.borrow().value() {
        Cell::Value(n) => s.push_str(&n.to_string()),
        Cell::Children(list) => {
            s.push('[');
            for node in list {
                to_string(node, s);
                s.push(',');
            }
            s.remove(s.len() - 1);
            s.push(']');
        }
    }
}

fn compare(left: Rc<RefCell<Node<usize>>>, right: Rc<RefCell<Node<usize>>>) -> Ordering {
    let l = &*left.borrow();
    let r = &*right.borrow();

    match (&l.value(), &r.value()) {
        (Cell::Value(n_left), Cell::Value(n_right)) => {
            if n_left < n_right {
                return Ordering::Greater;
            }
            if n_left == n_right {
                return Ordering::Equal;
            }
            Ordering::Less
        }
        (Cell::Value(n_left), Cell::Children(_)) => {
            let mut left_list = Node::new_list();
            left_list.add_child(Node::new_value(*n_left));

            return compare(left_list, right.clone());
        }
        (Cell::Children(_), Cell::Value(n_right)) => {
            let mut right_list = Node::new_list();
            right_list.add_child(Node::new_value(*n_right));

            compare(left.clone(), right_list)
        }
        (Cell::Children(l), Cell::Children(r)) => {
            let mut left_iter = l.iter();
            let mut right_iter = r.iter();

            loop {
                match (left_iter.next(), right_iter.next()) {
                    (None, None) => return Ordering::Equal,
                    (None, Some(_)) => return Ordering::Greater,
                    (Some(_), None) => return Ordering::Less,
                    (Some(l), Some(r)) => match compare(l.clone(), r.clone()) {
                        Ordering::Equal => (),
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                    },
                }
            }
        }
    }
}

fn line_to_tree(str: &str) -> Rc<RefCell<Node<usize>>> {
    let root = Node::new_list();
    let mut current = root.clone();
    let mut iterator = str.chars().into_iter().peekable();

    iterator.next();
    iterator.next_back();

    while let Some(c) = iterator.next() {
        if c == '[' {
            let new_list = Node::new_list();
            let n = new_list.clone();
            current.add_child(new_list);
            current = n;
        } else if c == ']' {
            current = current.parent();
        } else if c == ',' {
        } else {
            let n = read_number(c, &mut iterator);
            current.add_child(Node::new_value(n));
        }
    }

    root
}

fn read_number<I>(c: char, chars: &mut Peekable<I>) -> usize
where
    I: Iterator<Item = char>,
{
    let mut n = (c as usize) - 48;
    while let Some(c) = chars.next_if(|c| c.is_ascii_digit()) {
        n *= 10;
        n += (c as usize) - 48;
    }
    n
}
