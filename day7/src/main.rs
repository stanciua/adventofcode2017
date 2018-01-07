#[macro_use]
extern crate nom;

use std::cell::RefCell;
use std::str;
use nom::{alphanumeric, space};
use nom::IResult::Done;
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};

named!(name<&str>, map_res!(alphanumeric, str::from_utf8));
named!(
    weight<i32>,
    map_res!(
        map_res!(
            delimited!(char!('('), is_not!(")"), char!(')')),
            str::from_utf8
        ),
        str::parse::<i32>
    )
);

named!(
    children<Vec<&str>>,
    map!(
        separated_nonempty_list_complete!(tag!(", "), alphanumeric),
        |vec: Vec<_>| vec.into_iter()
            .map(|v| str::from_utf8(v).unwrap())
            .collect()
    )
);
#[derive(Debug, Eq, PartialEq)]
struct Program<'a> {
    name: &'a str,
    weight: i32,
    children: Option<Vec<&'a str>>,
}

named!(
    program<Program>,
    do_parse!(
        n: name >> space >> w: weight >> alt!(eof!() | tag!(" -> ")) >> c: opt!(children)
            >> (Program {
                name: n,
                weight: w,
                children: c,
            })
    )
);

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_name() {
        assert_eq!(name(b"pbga (66)"), Done(&b" (66)"[..], "pbga"));
    }

    #[test]
    fn test_weight() {
        assert_eq!(weight(b"(66)"), Done(&b""[..], 66));
    }
    #[test]
    fn test_children() {
        assert_eq!(
            children(b"gyxo, ebii, jptl"),
            Done(&b""[..], vec!["gyxo", "ebii", "jptl"])
        );
    }
    #[test]
    fn test_single_program() {
        assert_eq!(
            program(b"gyxo (61)"),
            Done(
                &b""[..],
                Program {
                    name: "gyxo",
                    weight: 61,
                    children: None,
                }
            )
        );
    }
    #[test]
    fn test_single_program_with_children() {
        assert_eq!(
            program(b"ugml (68) -> gyxo, ebii, jptl"),
            Done(
                &b""[..],
                Program {
                    name: "ugml",
                    weight: 68,
                    children: Some(vec!["gyxo", "ebii", "jptl"]),
                }
            )
        );
    }
}
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
        if let Done(_, o) = program(line.as_bytes()) {
            programs.push(o);
        }
    }
    let bottom_prog = bottom_program(&programs);

    println!("Bottom program: {:?}", bottom_prog);
    println!(
        "Needed weight is: {:?}",
        calculate_needed_weight(&bottom_prog, &programs)
    );
}

fn bottom_program<'a>(programs: &'a [Program]) -> String {
    // create a set of all names then create a set of all the children and do a
    // difference operation on them, that should give us the bottom program
    let all_programs = programs.iter().map(|p| p.name).collect::<HashSet<_>>();
    let all_children = programs
        .iter()
        .filter(|p| p.children.is_some())
        .flat_map(|p| p.children.as_ref().unwrap().iter().cloned())
        .collect::<HashSet<_>>();
    let all_children = all_children.into_iter().collect::<HashSet<_>>();
    all_programs
        .difference(&all_children)
        .take(1)
        .next()
        .unwrap()
        .to_string()
}

fn sum_all_children<'a>(
    child: &str,
    name_program_map: &'a HashMap<&'a str, &'a Program>,
    sum: &mut i32,
) {
    if name_program_map[child].children.is_none() {
        return;
    }
    for c in name_program_map[child].children.clone().unwrap() {
        sum_all_children(c, name_program_map, sum);
        let weight = name_program_map[c].weight;
        *sum += weight;
    }
}

fn get_sub_towers<'a>(
    child: &'a str,
    name_program_map: &'a HashMap<&'a str, &'a Program>,
    sub_towers: &mut RefCell<Vec<&'a str>>,
) {
    if name_program_map[child].children.is_none() {
        return;
    }
    sub_towers.borrow_mut().push(child);
    for c in name_program_map[child].children.clone().unwrap() {
        get_sub_towers(c, name_program_map, sub_towers);
    }
}
fn calculate_needed_weight<'a>(bottom_prog: &str, programs: &'a [Program]) -> i32 {
    let name_prog_map = programs.iter().fold(HashMap::new(), |mut acc, v| {
        acc.insert(v.name, v);
        acc
    });

    let mut sub_towers = RefCell::new(Vec::new());
    get_sub_towers(bottom_prog, &name_prog_map, &mut sub_towers);
    // check all sub-towers and see wich one is not balanced
    let sub_towers_with_weights = sub_towers
        .into_inner()
        .into_iter()
        .map(|t| (t, name_prog_map[t].children.clone().unwrap()))
        .map(|t| {
            (
                t.0,
                t.1
                    .into_iter()
                    .map(|t| {
                        let mut sum = 0;
                        sum_all_children(t, &name_prog_map, &mut sum);
                        sum += name_prog_map[t].weight;
                        (t, sum)
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    // we need to go from bottom to up, first tower that has the first wrong weight is
    // the unbalanced tower
    for (_, t) in sub_towers_with_weights.into_iter().rev() {
        if let Some(unbalanced_tower) = get_unbalanced_data(t.as_slice()) {
            let mut weights = vec![name_prog_map[(unbalanced_tower.0).0].weight];
            weights.extend(
                name_prog_map[(unbalanced_tower.0).0]
                    .children
                    .clone()
                    .unwrap()
                    .into_iter()
                    .map(|v| {
                        let mut sum = 0;
                        sum_all_children(v, &name_prog_map, &mut sum);
                        sum += name_prog_map[v].weight;
                        sum
                    })
                    .collect::<Vec<_>>(),
            );

            // get the tower with unbalanced weight
            let unbalanced_tower_weigth = weights
                .iter()
                .fold(HashMap::new(), |mut acc, v| {
                    {
                        let counter = acc.entry(v).or_insert(0);
                        *counter += 1;
                    }
                    acc
                })
                .into_iter()
                .find(|&(_, v)| v == 1)
                .unwrap()
                .0;
            return unbalanced_tower_weigth - ((unbalanced_tower.0).1 - unbalanced_tower.1).abs();
        }
    }
    0
}

fn get_unbalanced_data<'a>(towers: &'a [(&'a str, i32)]) -> Option<((&'a str, i32), i32)> {
    let tower_weight_counter = towers
        .iter()
        .map(|&v| v.1)
        .fold(HashMap::new(), |mut acc, v| {
            {
                let counter = acc.entry(v).or_insert(0);
                *counter += 1;
            }
            acc
        });

    if tower_weight_counter
        .iter()
        .find(|&(_, &v)| v == 1)
        .is_some()
    {
        let unbalanced_weight = tower_weight_counter
            .iter()
            .find(|&(_, &v)| v == 1)
            .unwrap()
            .0;
        let weight = tower_weight_counter
            .iter()
            .find(|&(_, &v)| v != 1)
            .unwrap()
            .0;

        // need to get the tower name that is not balanced
        let name = towers
            .iter()
            .find(|&v| v.1 == *unbalanced_weight)
            .unwrap()
            .0;
        return Some(((name, *unbalanced_weight), *weight));
    }
    None
}
