#[macro_use]
extern crate nom;

use std::str;
use nom::{digit, space};
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

named!(
    program<i32>,
    map_res!(map_res!(digit, str::from_utf8), str::parse)
);
named!(
    peers<HashSet<i32>>,
    map!(
        separated_nonempty_list_complete!(tag!(", "), digit),
        |vec: Vec<_>| vec.into_iter()
            .map(|v| str::from_utf8(v).unwrap())
            .map(|s| str::parse::<i32>(s).unwrap())
            .collect()
    )
);

named!(
    program_peers<(i32, HashSet<i32>)>,
    do_parse!(p: program >> space >> tag!("<->") >> space >> ps: peers >> (p, ps))
);

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut programs = Vec::new();
    for line in input_txt.lines() {
        let (_, peers) = program_peers(line.as_bytes()).unwrap().1;
        programs.push(peers);
    }

    println!(
        "Number of programs connected to program `0` is: {:?}",
        get_connected_programs(0, &programs).len()
    );
    println!("Number of groups: {:?}", get_num_of_groups(&programs));
}

fn get_num_of_groups(programs: &[HashSet<i32>]) -> i32 {
    let mut remaining_progs = (0i32..programs.len() as i32)
        .into_iter()
        .collect::<HashSet<_>>();

    remaining_progs = remaining_progs
        .difference(&get_connected_programs(0, &programs))
        .cloned()
        .collect::<HashSet<_>>();

    let mut groups = 1;
    while remaining_progs.len() != 0 {
        let prog_id = *remaining_progs.iter().take(1).next().unwrap();
        let connected_progrs = get_connected_programs(prog_id, &programs);
        remaining_progs = remaining_progs
            .difference(&connected_progrs)
            .cloned()
            .collect::<HashSet<_>>();
        groups += 1;
    }
    groups
}
// we can use a tree to do this, an extra dependency should be added
fn get_connected_programs(program: i32, programs: &[HashSet<i32>]) -> HashSet<i32> {
    // Tracks already visited programs
    let mut seen_programs = HashSet::new();
    // We start tracking with program 0
    seen_programs.insert(program);
    // Holds all the programs that communicate up to program `program`
    let mut conn_progs = programs[program as usize]
        .iter()
        .cloned()
        .collect::<HashSet<_>>();
    conn_progs.extend(seen_programs.iter());

    loop {
        let mut peers: HashSet<i32> = HashSet::new();
        let diff = conn_progs
            .difference(&seen_programs)
            .cloned()
            .collect::<HashSet<_>>();
        for p in &diff {
            peers.extend(programs[*p as usize].iter());
        }
        seen_programs.extend(diff);
        conn_progs.extend(peers.iter());

        if seen_programs.len() == conn_progs.len() {
            break;
        }
    }
    conn_progs
}
