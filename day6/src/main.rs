use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    if let Ok(_) = input.read_to_string(&mut input_txt) {
        let list = input_txt
            .split_whitespace()
            .map(|word| word.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let (steps, cycle_list) = count_redis_steps(&list);
        println!("Redistribution steps: {}", steps);
        println!(
            "Infinite loops cycle steps: {:?}",
            count_cycles_infinite_loop((cycle_list.as_slice()))
        );
    }
}

fn count_redis_steps(list: &[i32]) -> (i32, Vec<i32>) {
    // keep the first list in memory
    let mut curr_list = list.iter().cloned().collect::<Vec<_>>();
    let len = curr_list.len();
    let mut memory: HashSet<Vec<i32>> = HashSet::new();
    let mut no_steps = 1;
    loop {
        if let Some((idx, &max_val)) = curr_list
            .iter()
            .enumerate()
            .rev()
            .max_by(|x, y| x.1.cmp(y.1))
        {
            let copy = curr_list.iter().cloned().collect::<Vec<_>>();
            memory.insert(copy);

            let mut pos = idx;
            let mut max = max_val;

            // set the maximum value to zero
            curr_list[idx] = 0;

            while max > 0 {
                curr_list[(pos + 1) % len] += 1;
                max -= 1;
                pos = (pos + 1) % len;
            }
        }

        if memory.contains(&curr_list) {
            break;
        } else {
            no_steps += 1;
        }
    }

    (no_steps, curr_list)
}

fn count_cycles_infinite_loop(list: &[i32]) -> i32 {
    let mut curr_list = list.iter().cloned().collect::<Vec<_>>();
    let len = curr_list.len();
    let mut no_steps = 1;
    loop {
        if let Some((idx, &max_val)) = curr_list
            .iter()
            .enumerate()
            .rev()
            .max_by(|x, y| x.1.cmp(y.1))
        {
            let mut pos = idx;
            let mut max = max_val;

            // set the maximum value to zero
            curr_list[idx] = 0;

            while max > 0 {
                curr_list[(pos + 1) % len] += 1;
                max -= 1;
                pos = (pos + 1) % len;
            }
        }

        if curr_list == list {
            break;
        } else {
            no_steps += 1;
        }
    }

    no_steps
}
