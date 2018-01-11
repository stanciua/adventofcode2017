#![feature(inclusive_range_syntax)]
#[macro_use]
extern crate nom;

use std::str;
use nom::{digit, space};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

named!(
    depth<i32>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    range<i32>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);

named!(
    firewall<(i32, i32)>,
    do_parse!(d: depth >> char!(':') >> space >> r: range >> (d, r))
);

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut firewall_map = HashMap::new();
    for line in input_txt.lines() {
        let (k, v) = firewall(line.as_bytes()).unwrap().1;
        firewall_map.insert(k, v);
    }

    let total_picoseconds = firewall_map.keys().cloned().max().unwrap();

    let firewall = firewall_map.iter().fold(
        vec![0; total_picoseconds as usize + 1],
        |mut acc, v| {
            acc[*v.0 as usize] = *v.1;
            acc
        },
    );

    let firewall = firewall
        .into_iter()
        .map(|v| {
            (0..v - 1)
                .into_iter()
                .chain((1..=v - 1).into_iter().rev())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // println!("{:?}", firewall);
    // println!(
    //     "The trip severity is: {:?}",
    //     get_trip_severity(0, &firewall)
    // );

    println!(
        "The fewest number of steps is: {:?}",
        get_the_fewest_number_of_steps(firewall.as_slice())
    );
}

fn get_the_fewest_number_of_steps(firewall: &[Vec<i32>]) -> usize {
    let mut delay = 0;
    let mut caught = get_trip_severity_two(delay, firewall);
    while caught.0 {
        delay += 1;
        caught = get_trip_severity_two(delay, firewall);
    }

    delay
}

fn get_trip_severity(delay: usize, firewall: &[Vec<i32>]) -> (bool, usize) {
    let mut trip_severity = 0;
    let mut caught = false;
    for (depth, ref ranges) in firewall.iter().enumerate() {
        if ranges.len() == 0 {
            continue;
        }
        let offset = (depth + delay) as usize;
        let scanner_pos = ranges
            .into_iter()
            .cycle()
            .skip(offset)
            .take(1)
            .next()
            .unwrap();

        if *scanner_pos == 0 {
            // we get caught by the scanner, we are at the top
            trip_severity += depth * range as usize;
            caught = true;
        }
    }

    (caught, trip_severity)
}

fn get_trip_severity_two(delay: usize, firewall: &[Vec<i32>]) -> (bool, usize) {
    println!("----------Delay: {} --------------", delay);

    let mut trip_severity = 0;
    let mut caught = false;
    for (depth, ref ranges) in firewall.iter().enumerate() {
        println!("Depth: {:?}", depth);

        if ranges.len() == 0 {
            println!("------------------------------");
            continue;
        }
        let offset = (depth + delay) as usize;
        println!("Offset: {}", offset);
        let scanner_pos = ranges.iter().cycle().skip(offset).take(1).next().unwrap();

        println!("Scanner Position: {}", scanner_pos);
        if *scanner_pos == 0 {
            // we get caught by the scanner, we are at the top
            trip_severity += depth * range as usize;
            println!("Caught");
            caught = true;
            break;
        }
        println!("------------------------------");
    }

    (caught, trip_severity)
}
