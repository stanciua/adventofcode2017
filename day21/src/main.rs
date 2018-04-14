#[macro_use]
extern crate nom;

use nom::*;
use std::fs::File;
use std::io::Read;
use std::str;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let mut rules = Vec::new();
    for line in input_txt.lines() {
        rules.push(rule(line.as_bytes()).unwrap().1);
    }
}
named!(
    chars<Vec<char>>,
    map!(
        map_res!(take_till!(|ch| ch != b'.' && ch != b'#'), str::from_utf8),
        |s| s.chars().collect::<Vec<_>>()
    )
);

named!(
    pixels<Vec<Vec<char>>>,
    separated_list_complete!(char!('/'), chars)
);

named!(
    rule<Rule>,
    do_parse!(p: pixels >> space >> tag!("=>") >> space >> r: pixels >> (Rule::from_pixels(p, r)))
);
// ##/## => ..#/#.#/..#
// .../.../... => .#../#..#/#.../.#..
// first row -> last column
// ...
// last row -> first column

#[derive(Debug)]
struct Rule {
    rule: Vec<Vec<char>>,
    enhancement: Vec<Vec<char>>,
}

impl Rule {
    fn from_pixels(rule: Vec<Vec<char>>, enhancement: Vec<Vec<char>>) -> Rule {
        Rule {
            rule: rule,
            enhancement: enhancement,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_chars_match() {
        assert_eq!(chars(b"##"), IResult::Done(&b""[..], vec!['#', '#']));
    }
    #[test]
    fn test_chars_separated_match() {
        assert_eq!(
            pixels(b"##/##"),
            IResult::Done(&b""[..], vec![vec!['#', '#'], vec!['#', '#']])
        );
    }
    #[test]
    fn rotate_by_90_degrees_2_by_2_matrix() {
        let mut input = vec![vec![1, 2], vec![3, 4]];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(input, vec![vec![3, 1], vec![4, 2]]);
    }
    #[test]
    fn rotate_by_90_degrees_3_by_3_matrix() {
        let mut input = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(input, vec![vec![7, 4, 1], vec![8, 5, 2], vec![9, 6, 3]]);
    }
    #[test]
    fn rotate_by_90_degrees_4_by_4_matrix() {
        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(
            input,
            vec![
                vec![13, 9, 5, 1],
                vec![14, 10, 6, 2],
                vec![15, 11, 7, 3],
                vec![16, 12, 8, 4],
            ]
        );
    }
    #[test]
    fn rotate_by_90_degrees_5_by_5_matrix() {
        let mut input = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        rotate_clockwise_90_deg(input.as_mut_slice());
        assert_eq!(
            input,
            vec![
                vec![21, 16, 11, 6, 1],
                vec![22, 17, 12, 7, 2],
                vec![23, 18, 13, 8, 3],
                vec![24, 19, 14, 9, 4],
                vec![25, 20, 15, 10, 5],
            ]
        );
    }
    #[test]
    fn test_flip_ud_2d() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![3, 4], vec![1, 2]]);
    }
    #[test]
    fn test_flip_ud_3d() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![7, 8, 9], vec![4, 5, 6], vec![1, 2, 3]]);
    }
    #[test]
    fn test_flip_ud_4d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![13, 14, 15, 16],
                vec![9, 10, 11, 12],
                vec![5, 6, 7, 8],
                vec![1, 2, 3, 4],
            ]
        );
    }
    #[test]
    fn test_flip_ud_5d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        flip_ud(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![21, 22, 23, 24, 25],
                vec![16, 17, 18, 19, 20],
                vec![11, 12, 13, 14, 15],
                vec![6, 7, 8, 9, 10],
                vec![1, 2, 3, 4, 5],
            ]
        );
    }

    // flip_lr tests
    #[test]
    fn test_flip_lr_2d() {
        let mut matrix = vec![vec![1, 2], vec![3, 4]];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![2, 1], vec![4, 3]]);
    }
    #[test]
    fn test_flip_lr_3d() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(matrix, vec![vec![3, 2, 1], vec![6, 5, 4], vec![9, 8, 7]]);
    }
    #[test]
    fn test_flip_lr_4d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4],
            vec![5, 6, 7, 8],
            vec![9, 10, 11, 12],
            vec![13, 14, 15, 16],
        ];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![4, 3, 2, 1],
                vec![8, 7, 6, 5],
                vec![12, 11, 10, 9],
                vec![16, 15, 14, 13],
            ]
        );
    }
    #[test]
    fn test_flip_lr_5d() {
        let mut matrix = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        flip_lr(matrix.as_mut_slice());
        assert_eq!(
            matrix,
            vec![
                vec![5, 4, 3, 2, 1],
                vec![10, 9, 8, 7, 6],
                vec![15, 14, 13, 12, 11],
                vec![20, 19, 18, 17, 16],
                vec![25, 24, 23, 22, 21],
            ]
        );
    }
}

fn rotate_clockwise_90_deg(input: &mut [Vec<i32>]) {
    let size = input.len();

    let no_cycles = (size as f64 / 2.0).floor() as usize;

    let mut width = size;
    for c in 0..no_cycles {
        for i in 0..width - 1 {
            let tmp = input[c + i][c];
            input[c + i][c] = input[width + c - 1][c + i];
            input[width + c - 1][c + i] = input[width + c - i - 1][width + c - 1];
            input[width + c - i - 1][width + c - 1] = input[c][width - i + c - 1];
            input[c][width - i + c - 1] = tmp;
        }
        width -= 2;
    }
}

fn flip_ud(matrix: &mut [Vec<i32>]) {
    let lgth = matrix.len();
    for idx in 0..lgth / 2 {
        assert!(matrix[idx].len() == matrix[lgth - idx - 1].len());
        let row_lgth = matrix[idx].len();
        for i in 0..row_lgth {
            let tmp = matrix[idx][i];
            matrix[idx][i] = matrix[lgth - idx - 1][i];
            matrix[lgth - idx - 1][i] = tmp;
        }
    }
}

fn flip_lr(matrix: &mut [Vec<i32>]) {
    let lgth = matrix.len();
    for idx in 0..lgth / 2 {
        let row_lgth = matrix.len();
        for i in 0..row_lgth {
            let tmp = matrix[i][idx];
            matrix[i][idx] = matrix[i][lgth - idx - 1];
            matrix[i][lgth - idx - 1] = tmp;
        }
    }
}
