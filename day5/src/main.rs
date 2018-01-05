use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let mut list = buffered
        .lines()
        .map(|word| word.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut copy_list = list.clone();

    println!(
        "Steps version one: {:?}",
        count_steps(&mut list, Version::One)
    );
    println!(
        "Steps version two: {:?}",
        count_steps(&mut copy_list, Version::Two)
    );
}

#[derive(Debug, PartialEq)]
enum Version {
    One,
    Two,
}
fn count_steps(list: &mut [i32], ver: Version) -> i32 {
    let mut curr_pos = 0;
    let mut no_steps = 1;
    let len = list.len();

    loop {
        if curr_pos + list[curr_pos as usize] < len as i32
            && curr_pos + list[curr_pos as usize] >= 0
        {
            let last_pos = curr_pos;
            curr_pos += list[curr_pos as usize];
            if ver == Version::Two && list[last_pos as usize] >= 3 {
                list[last_pos as usize] -= 1;
            } else {
                list[last_pos as usize] += 1;
            }
            no_steps += 1;
        } else {
            break;
        }
    }

    no_steps
}
