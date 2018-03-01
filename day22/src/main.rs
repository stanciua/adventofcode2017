use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Flag {
    Clean,
    Flagged,
    Infected,
    Weakened,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Algorithm {
    V1,
    V2,
}
fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let initial_grid = input_txt
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!(
        "The number of infections after {} bursts is: {}",
        10_000,
        advance(
            Algorithm::V1,
            initial_grid.as_slice(),
            initial_grid[0].len(),
            10_000
        )
    );

    println!(
        "The number of infections after {} bursts is: {}",
        10_000_000,
        advance(
            Algorithm::V2,
            initial_grid.as_slice(),
            initial_grid[0].len(),
            10_000_000
        )
    );
}

fn extend_grid(start_grid: &[Vec<char>], size: usize) -> Vec<Vec<char>> {
    let mut grid = Vec::with_capacity(size);

    let start_grid_size = start_grid[0].len();
    for _ in 0..size {
        let mut line = vec!['.'; size];
        grid.push(line);
    }

    for i in 0..start_grid_size {
        for j in 0..start_grid_size {
            let (ni, nj) = translate_point((i, j), start_grid_size, size);
            grid[ni][nj] = start_grid[i][j];
        }
    }
    grid
}

fn translate_point(point: (usize, usize), from: usize, to: usize) -> (usize, usize) {
    let middle_from = (from / 2, from / 2);
    let middle_to = (to / 2, to / 2);
    let up_left = (middle_to.0 - middle_from.0, middle_to.1 - middle_from.1);
    (up_left.0 + point.0, up_left.1 + point.1)
}

fn advance(algorithm: Algorithm, from_grid: &[Vec<char>], size: usize, bursts: usize) -> usize {
    let mut curr_dir = Direction::Up;
    let mut grid = extend_grid(from_grid, size);
    let mut curr_pos = (size / 2, size / 2);
    let mut infection_count = 0;
    for _ in 0..bursts {
        let grid_size = grid[0].len();
        // we need to make sure that the next position is not outside the
        // boundaries of the grid, if this happens we need to extend the grid and
        // translate the position
        if curr_pos.0 == 0 || curr_pos.0 == grid_size - 1 || curr_pos.1 == 0
            || curr_pos.1 == grid_size - 1
        {
            grid = extend_grid(grid.as_slice(), grid_size + 2);
            curr_pos = translate_point(curr_pos, grid_size, grid_size + 2);
        }

        let mut state = match grid[curr_pos.0][curr_pos.1] {
            '#' => Flag::Infected,
            '.' => Flag::Clean,
            'W' => Flag::Weakened,
            'F' => Flag::Flagged,
            _ => panic!("invalid value detected!"),
        };
        let (mut next_pos, next_dir) = get_next_pos(curr_pos, curr_dir, state);
        curr_dir = next_dir;

        match state {
            Flag::Clean => if algorithm == Algorithm::V1 {
                grid[curr_pos.0][curr_pos.1] = '#';
                infection_count += 1;
            } else {
                grid[curr_pos.0][curr_pos.1] = 'W'
            },
            Flag::Flagged => grid[curr_pos.0][curr_pos.1] = '.',
            Flag::Infected => if algorithm == Algorithm::V1 {
                grid[curr_pos.0][curr_pos.1] = '.'
            } else {
                grid[curr_pos.0][curr_pos.1] = 'F'
            },

            Flag::Weakened => {
                grid[curr_pos.0][curr_pos.1] = '#';
                infection_count += 1;
            }
        }
        curr_pos = next_pos;
    }

    infection_count
}

fn get_next_pos(
    curr_pos: (usize, usize),
    curr_dir: Direction,
    state: Flag,
) -> ((usize, usize), Direction) {
    match curr_dir {
        Direction::Up => match state {
            Flag::Clean => ((curr_pos.0, curr_pos.1 - 1), Direction::Left),
            Flag::Weakened => ((curr_pos.0 - 1, curr_pos.1), Direction::Up),
            Flag::Infected => ((curr_pos.0, curr_pos.1 + 1), Direction::Right),
            Flag::Flagged => ((curr_pos.0 + 1, curr_pos.1), Direction::Down),
        },
        Direction::Down => match state {
            Flag::Clean => ((curr_pos.0, curr_pos.1 + 1), Direction::Right),
            Flag::Weakened => ((curr_pos.0 + 1, curr_pos.1), Direction::Down),
            Flag::Infected => ((curr_pos.0, curr_pos.1 - 1), Direction::Left),
            Flag::Flagged => ((curr_pos.0 - 1, curr_pos.1), Direction::Up),
        },
        Direction::Left => match state {
            Flag::Clean => ((curr_pos.0 + 1, curr_pos.1), Direction::Down),
            Flag::Weakened => ((curr_pos.0, curr_pos.1 - 1), Direction::Left),
            Flag::Infected => ((curr_pos.0 - 1, curr_pos.1), Direction::Up),
            Flag::Flagged => ((curr_pos.0, curr_pos.1 + 1), Direction::Right),
        },
        Direction::Right => match state {
            Flag::Clean => ((curr_pos.0 - 1, curr_pos.1), Direction::Up),
            Flag::Weakened => ((curr_pos.0, curr_pos.1 + 1), Direction::Right),
            Flag::Infected => ((curr_pos.0 + 1, curr_pos.1), Direction::Down),
            Flag::Flagged => ((curr_pos.0, curr_pos.1 - 1), Direction::Left),
        },
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_translate_point_for_origin() {
        assert_eq!(translate_point((1, 1), 3, 5), (2, 2));
    }
    #[test]
    fn test_translate_point_for_up_left() {
        assert_eq!(translate_point((0, 0), 3, 5), (1, 1));
    }
    #[test]
    fn test_translate_point_for_down_right() {
        assert_eq!(translate_point((2, 2), 3, 5), (3, 3));
    }

}
