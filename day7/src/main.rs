// pbga (66)
// xhth (57)
// ebii (61)
// havc (66)
// ktlj (57)
// fwft (72) -> ktlj, cntj, xhth
// qoyq (66)
// padx (45) -> pbga, havc, qoyq
// tknk (41) -> ugml, padx, fwft
// jptl (61)
// ugml (68) -> gyxo, ebii, jptl
// gyxo (61)
// cntj (57)
#[macro_use]
extern crate nom;

use std::str;
use nom::{alphanumeric, space};
use nom::IResult::Done;
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

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
        |vec: Vec<_>| {
            vec.into_iter()
                .map(|v| str::from_utf8(v).unwrap())
                .collect()
        }
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
    use nom::IResult::*;
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

    println!("Bottom program: {:?}", bottom_program(&programs));
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
