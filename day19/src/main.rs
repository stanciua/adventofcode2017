use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let path = "input.txt";
    let input = File::open(path).expect("Unable to open file!");
    let buffered = BufReader::new(input);

    let lines = buffered.lines().map(|l| l.unwrap()).collect::<Vec<_>>();

    let mut map: Vec<Vec<char>> = Vec::with_capacity(lines.len());
    for line in lines.iter() {
        map.push(line.chars().collect::<Vec<_>>());
    }

    // start position is always the first line a the top with a single | on it
    let start_pos = (0, map[0].iter().position(|&c| c == '|').unwrap());

    println!(
        "Path to exit is: {:?}",
        find_exit(map.as_slice(), start_pos)
    );
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn find_exit(map: &[Vec<char>], start_position: (usize, usize)) -> String {
    let mut curr_dir = Direction::Down;
    let mut curr_pos = start_position;
    let mut output = String::new();

    loop {
        let char_at_pos = map[curr_pos.0][curr_pos.1];
        if char_at_pos.is_alphanumeric() {
            output.push(char_at_pos);
        }
        if let Some((next_dir, next_pos)) = get_next_direction(map, curr_dir, curr_pos) {
            curr_dir = next_dir;
            curr_pos = next_pos;
        } else {
            break;
        }
    }

    output
}

fn get_next_direction(
    map: &[Vec<char>],
    curr_dir: Direction,
    curr_pos: (usize, usize),
) -> Option<(Direction, (usize, usize))> {
    let (rows, cols) = (map.len(), map[0].len());
    match curr_dir {
        Direction::Down => {
            // Down
            if curr_pos.0 + 1 < rows && map[curr_pos.0 + 1][curr_pos.1] != ' ' {
                return Some((Direction::Down, (curr_pos.0 + 1, curr_pos.1)));
            }
            // Left
            if curr_pos.1 as i32 - 1 >= 0 && map[curr_pos.0][curr_pos.1 - 1] != ' ' {
                return Some((Direction::Left, (curr_pos.0, curr_pos.1 - 1)));
            }
            // Right
            if curr_pos.1 + 1 < cols && map[curr_pos.0][curr_pos.1 + 1] != ' ' {
                return Some((Direction::Right, (curr_pos.0, curr_pos.1 + 1)));
            }
        }
        Direction::Up => {
            // Up
            if curr_pos.0 as i32 - 1 >= 0 && map[curr_pos.0 - 1][curr_pos.1] != ' ' {
                return Some((Direction::Up, (curr_pos.0 - 1, curr_pos.1)));
            }
            // Left
            if curr_pos.1 as i32 - 1 >= 0 && map[curr_pos.0][curr_pos.1 - 1] != ' ' {
                return Some((Direction::Left, (curr_pos.0, curr_pos.1 - 1)));
            }
            // Right
            if curr_pos.1 + 1 < cols && map[curr_pos.0][curr_pos.1 + 1] != ' ' {
                return Some((Direction::Right, (curr_pos.0, curr_pos.1 + 1)));
            }
        }
        Direction::Left => {
            // Left
            if curr_pos.1 as i32 - 1 >= 0 && map[curr_pos.0][curr_pos.1 - 1] != ' ' {
                return Some((Direction::Left, (curr_pos.0, curr_pos.1 - 1)));
            }

            // Down
            if curr_pos.0 + 1 < rows && map[curr_pos.0 + 1][curr_pos.1] != ' ' {
                return Some((Direction::Down, (curr_pos.0 + 1, curr_pos.1)));
            }
            // Up
            if curr_pos.0 as i32 - 1 >= 0 && map[curr_pos.0 - 1][curr_pos.1] != ' ' {
                return Some((Direction::Up, (curr_pos.0 - 1, curr_pos.1)));
            }
        }
        Direction::Right => {
            // Right
            if curr_pos.1 + 1 < cols && map[curr_pos.0][curr_pos.1 + 1] != ' ' {
                return Some((Direction::Right, (curr_pos.0, curr_pos.1 + 1)));
            }

            // Down
            if curr_pos.0 + 1 < rows && map[curr_pos.0 + 1][curr_pos.1] != ' ' {
                return Some((Direction::Down, (curr_pos.0 + 1, curr_pos.1)));
            }
            // Up
            if curr_pos.0 as i32 - 1 >= 0 && map[curr_pos.0 - 1][curr_pos.1] != ' ' {
                return Some((Direction::Up, (curr_pos.0 - 1, curr_pos.1)));
            }
        }
    }
    None
}
