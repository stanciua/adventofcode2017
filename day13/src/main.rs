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

    let mut firewall_policy = HashMap::new();
    for line in input_txt.lines() {
        let (k, v) = firewall(line.as_bytes()).unwrap().1;
        firewall_policy.insert(k, v);
    }

    println!(
        "The trip severity is: {:?}",
        get_trip_severity(&mut firewall_policy)
    );
}

fn get_trip_severity(firewall: &mut HashMap<i32, i32>) -> i32 {
    let mut trip_severity = 0;
    let total_picoseconds = firewall.keys().cloned().max().unwrap();
    for ps in 0..total_picoseconds + 1 {
        let depth = ps;
        let &mut range = firewall.entry(ps).or_insert(0);
        if range == 0 {
            continue;
        }
        let scanner_pos = if ps > range - 1 { ps % (range - 1) } else { ps };
        if scanner_pos == 0 {
            // we get caught by the scanner, we are at the top
            trip_severity += depth * range;
        }
    }
    trip_severity
}
