// Solution taken from a C++ solution found here: https://www.reddit.com/r/adventofcode/comments/7lte5z/2017_day_24_solutions/
use std::cmp::Ord;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Component {
    port1: u32,
    port2: u32,
}

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut ports = Vec::new();
    for line in input_txt.lines() {
        let p = line.split('/').collect::<Vec<_>>();
        ports.push(Component {
            port1: p[0].parse::<u32>().unwrap(),
            port2: p[1].parse::<u32>().unwrap(),
        });
    }
    let mut max_overall_strength = 0;
    let mut max_length = 0;
    let mut max_strength_among_longest = 0;
    let mut used_ports = ports.iter().map(|p| (*p, false)).collect::<HashMap<_, _>>();
    calculate_strongest_bridge(
        &ports,
        &mut used_ports,
        0,
        0,
        0,
        &mut max_overall_strength,
        &mut max_length,
        &mut max_strength_among_longest,
    );

    println!("Strongest bridge is: {:?}", max_overall_strength);
    println!(
        "Strongest bridge amont the longest is: {:?}",
        max_strength_among_longest
    );
}

fn calculate_strongest_bridge(
    ports: &[Component],
    used_ports: &mut HashMap<Component, bool>,
    port: u32,
    length: u32,
    strength: u32,
    max_overall_strength: &mut u32,
    max_length: &mut u32,
    max_strength_among_longest: &mut u32,
) {
    *max_overall_strength = strength.max(*max_overall_strength);
    *max_length = length.max(*max_length);

    if length == *max_length {
        *max_strength_among_longest = strength.max(*max_strength_among_longest);
    }

    for p in ports {
        if !used_ports[p] && (p.port1 == port || p.port2 == port) {
            *used_ports.entry(*p).or_insert(true) = true;
            calculate_strongest_bridge(
                ports,
                used_ports,
                if p.port1 == port {
                    p.port2
                } else {
                    p.port1
                },
                length + 1,
                strength + p.port1 + p.port2,
                max_overall_strength,
                max_length,
                max_strength_among_longest,
            );
            *used_ports.entry(*p).or_insert(false) = false;
        }
    }
}
