#![feature(slice_rotate)]
use std::fs::File;
use std::io::prelude::*;
use std::str::FromStr;

#[derive(Debug)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Move {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().take(1).next().unwrap() {
            's' => Ok(Move::Spin(
                s.chars()
                    .skip(1)
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap(),
            )),
            'x' => {
                let positions = s.chars()
                    .skip(1)
                    .collect::<String>()
                    .split('/')
                    .map(|v| v.parse::<usize>().unwrap())
                    .collect::<Vec<_>>();

                Ok(Move::Exchange(positions[0], positions[1]))
            }
            'p' => {
                let partners = s.chars()
                    .skip(1)
                    .collect::<String>()
                    .split('/')
                    .map(|s| s.chars().take(1).next().unwrap())
                    .collect::<Vec<_>>();
                Ok(Move::Partner(partners[0], partners[1]))
            }
            _ => Err("Unsupported value found"),
        }
    }
}
fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    if let Ok(_) = input.read_to_string(&mut input_txt) {
        let moves = input_txt
            .split(',')
            .map(|m| Move::from_str(m).unwrap())
            .collect::<Vec<Move>>();

        let mut line = "abcdefghijklmnop".chars().collect::<Vec<_>>();

        execute_move(line.as_mut_slice(), &moves);

        println!(
            "The program order after dance is: {:?}",
            line.into_iter().collect::<String>()
        );

        let mut line = "abcdefghijklmnop".chars().collect::<Vec<_>>();

        println!(
            "The program order after 1_000_000_000 dances is: {:?}",
            order_after_billion_dances(line.as_mut_slice(), &moves)
        );
    }
}

fn order_after_billion_dances(line: &mut [char], moves: &[Move]) -> String {
    let mut dances = Vec::new();
    let mut no_iterations = 0;
    for idx in 0..1_000_000_000 {
        execute_move(line, &moves);
        dances.push(line.iter().collect::<String>());
        if line
            == &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'
            ] {
            no_iterations = idx + 1;
            break;
        }
    }

    dances[(1_000_000_000 % no_iterations) - 1].clone()
}

fn execute_move(line: &mut [char], moves: &[Move]) {
    for m in moves {
        match *m {
            Move::Spin(n) => {
                let len = line.len();
                line.rotate(len - n);
            }
            Move::Exchange(pos_a, pos_b) => {
                line.swap(pos_a, pos_b);
            }
            Move::Partner(a, b) => {
                let pos_a = line.iter().position(|&p| p == a).unwrap();
                let pos_b = line.iter().position(|&p| p == b).unwrap();
                line.swap(pos_a, pos_b);
            }
        }
    }
}
