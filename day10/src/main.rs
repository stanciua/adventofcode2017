#![feature(inclusive_range_syntax)]

use std::str;
use std::fs::File;
use std::io::Read;

fn get_result_of_multiply_first_two_num(input: &str) -> i32 {
    let lengths = input
        .split(',')
        .map(|s| str::parse::<u8>(s).unwrap())
        .collect::<Vec<_>>();

    let mut list = (0u8..=255).collect::<Vec<_>>();
    let mut curr_pos = 0;
    let mut skip_size = 0;

    run_iteration(
        list.as_mut_slice(),
        lengths.as_slice(),
        &mut curr_pos,
        &mut skip_size,
    );
    list.iter()
        .cloned()
        .take(2)
        .map(|v| v as i32)
        .product::<i32>()
}

fn get_hasher_from_input(input: &str) -> String {
    let mut lengths = input.as_bytes().iter().cloned().collect::<Vec<_>>();
    lengths.extend(vec![17u8, 31, 73, 47, 23]);

    let mut curr_pos = 0;
    let mut skip_size = 0;
    let mut sparse_hash = (0..=255).into_iter().collect::<Vec<_>>();

    for _ in 0..64 {
        run_iteration(
            sparse_hash.as_mut_slice(),
            lengths.as_slice(),
            &mut curr_pos,
            &mut skip_size,
        );
    }

    let mut dense_hash = Vec::new();
    for slice in sparse_hash.chunks(16) {
        dense_hash.push(slice.iter().fold(0, |mut acc, v| {
            acc ^= v;
            acc
        }));
    }
    dense_hash
        .iter()
        .map(|v| format!("{:02x}", v))
        .fold(String::new(), |mut acc, v| {
            acc.extend(v.chars());
            acc
        })
}
fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    println!(
        "Result is: {:?}",
        get_result_of_multiply_first_two_num(&input_txt)
    );

    println!("Knot hash is: {:?}", get_hasher_from_input(&input_txt));
}

fn run_iteration(list: &mut [u8], lengths: &[u8], curr_pos: &mut usize, skip_size: &mut usize) {
    let len = list.len();
    lengths
        .iter()
        .fold((list, curr_pos, skip_size), |state, &n| {
            let sublist = state
                .0
                .iter()
                .cycle()
                .skip(*state.1 as usize)
                .take(len)
                .cloned()
                .collect::<Vec<_>>();

            let mut rev_sublist = sublist
                .iter()
                .take(n as usize)
                .collect::<Vec<_>>()
                .into_iter()
                .cloned()
                .rev()
                .collect::<Vec<_>>();
            rev_sublist.extend(sublist.into_iter().skip(n as usize).take(len - n as usize));

            for (idx, e) in rev_sublist.into_iter().enumerate() {
                state.0[(*state.1 + idx) % len] = e;
            }

            *state.1 = (*state.1 + n as usize + *state.2) % len;
            *state.2 = (*state.2 + 1) % len;
            state
        });
}
