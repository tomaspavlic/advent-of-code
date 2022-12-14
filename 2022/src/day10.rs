use std::fs;

fn main() {
    let input_file_content = fs::read_to_string("input/day10.txt").unwrap();

    let mut cycle: i32 = 0;
    let mut register: i32 = 1;
    let mut sum: i32 = 0;
    let mut display = vec![vec!['.'; 40]; 6];

    for op in input_file_content.lines() {
        if op == "noop" {
            cycle += 1;
            process_cycle(cycle, register, &mut display, &mut sum);
            continue;
        } 

        for _ in 0..2 {
            cycle += 1;
            process_cycle(cycle, register, &mut display, &mut sum);
        }
        
        let add_x: i32 = op.split(' ').next_back().unwrap().parse().unwrap();
        register += add_x;
    }

    println!("1.part: {}", sum);
    println!("2.part: ");
    for row in display {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn process_cycle(cycle: i32, register: i32, display: &mut [Vec<char>], sum: &mut i32) {
    let cycle_row = (cycle - 1) / 40;
    let cycle_col = cycle - (cycle_row * 40) - 1;

    let row_vec = display.get_mut(cycle_row as usize).unwrap();
    if register == cycle_col || register - 1 == cycle_col || register + 1 == cycle_col {
        if let Some(p) = row_vec.get_mut(cycle_col as usize) {
            *p = '#';
        }
    }

    if cycle == 20 || cycle == 60 || cycle == 100 || cycle == 140 || cycle == 180 || cycle == 220 {
        *sum += cycle * register;
    }
}
