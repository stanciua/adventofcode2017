#[macro_use]
extern crate nom;

use std::str;
use nom::{alpha, alphanumeric, space};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;
use std::result::Result;
use std::collections::HashMap;

named!(register<&str>, map_res!(alpha, str::from_utf8));
named!(
    operation<Operation>,
    map_res!(
        map_res!(alt!(tag!("inc") | tag!("dec")), str::from_utf8),
        Operation::from_str
    )
);
named!(
    amount<i32>,
    map_res!(map_res!(alphanumeric, str::from_utf8), str::parse)
);
named!(
    operator<Operator>,
    map_res!(
        map_res!(
            alt!(tag!("<=") | tag!(">=") | tag!("<") | tag!(">") | tag!("==") | tag!("!=")),
            str::from_utf8
        ),
        Operator::from_str
    )
);

named!(
    instruction<Instruction>,
    do_parse!(
        r: register >> space >> o: operation >> space >> m: opt!(char!('-')) >> a: amount
            >> dbg_dmp!(space) >> tag!("if") >> space >> ri: register >> space
            >> oi: operator >> dbg_dmp!(space) >> mi: opt!(char!('-')) >> v: amount
            >> (Instruction {
                register: r,
                operation: o,
                amount: if m.is_some() { -a } else { a },
                condition: (ri, oi, if mi.is_some() { -v } else { v }),
            })
    )
);
#[derive(Debug)]
enum Operator {
    Less,
    Greater,
    GreaterOrEqual,
    LessOrEqual,
    Equal,
    NotEqual,
}

impl FromStr for Operator {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(Operator::Less),
            ">" => Ok(Operator::Greater),
            "<=" => Ok(Operator::LessOrEqual),
            ">=" => Ok(Operator::GreaterOrEqual),
            "==" => Ok(Operator::Equal),
            "!=" => Ok(Operator::NotEqual),
            _ => Err("Unsupported operator"),
        }
    }
}

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

impl FromStr for Operation {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(Operation::Inc),
            "dec" => Ok(Operation::Dec),
            _ => Err("Unsupported operation"),
        }
    }
}

#[derive(Debug)]
struct Instruction<'a> {
    register: &'a str,
    operation: Operation,
    amount: i32,
    condition: (&'a str, Operator, i32),
}

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut instructions = Vec::new();
    for line in input_txt.lines() {
        instructions.push(instruction(line.as_bytes()).unwrap().1);
    }


    println!(
        "Largest value in any register is: {:?}",
        get_the_largest_val_in_any_reg_from(instructions.as_slice())
    );
}

fn get_the_largest_val_in_any_reg_from(instructions: &[Instruction]) -> i32 {
    let mut registers = instructions.iter().map(|i| i.register).fold(
        HashMap::new(),
        |mut acc, r| {
            acc.entry(r).or_insert(0);
            acc
        },
    );

    for instruction in instructions {
        // Evaluate the condition and see if this instruction is needed to be run or not
        let execute = match instruction.condition.1 {
            Operator::Less => registers[instruction.condition.0] < instruction.condition.2,
            Operator::Greater => registers[instruction.condition.0] > instruction.condition.2,
            Operator::GreaterOrEqual => {
                registers[instruction.condition.0] >= instruction.condition.2
            }
            Operator::LessOrEqual => registers[instruction.condition.0] <= instruction.condition.2,
            Operator::Equal => registers[instruction.condition.0] == instruction.condition.2,
            Operator::NotEqual => registers[instruction.condition.0] != instruction.condition.2,
        };
        // if we need to execute it adjust the register value with the proper amount
        if execute {
            let old_val = registers.entry(instruction.register).or_insert(0);
            match instruction.operation {
                Operation::Dec => *old_val -= instruction.amount,
                Operation::Inc => *old_val += instruction.amount,
            }
        }
    }

    *registers.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap().1
}
