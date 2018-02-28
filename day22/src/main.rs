use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
        move_virus(initial_grid.as_slice(), initial_grid[0].len(), 10_000)
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

fn move_virus(from_grid: &[Vec<char>], size: usize, bursts: usize) -> usize {
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
            grid = extend_grid(grid.as_slice(), grid_size * 2);
            curr_pos = translate_point(curr_pos, grid_size, grid_size * 2);
        }

        let mut infected = grid[curr_pos.0][curr_pos.1] == '#';
        let (mut next_pos, next_dir) = get_next_pos(curr_pos, curr_dir, infected);
        curr_dir = next_dir;

        if infected {
            grid[curr_pos.0][curr_pos.1] = '.';
        } else {
            grid[curr_pos.0][curr_pos.1] = '#';
            infection_count += 1;
        }

        curr_pos = next_pos;
    }

    infection_count
}

fn get_next_pos(
    curr_pos: (usize, usize),
    curr_dir: Direction,
    infected: bool,
) -> ((usize, usize), Direction) {
    match curr_dir {
        Direction::Up => if infected {
            ((curr_pos.0, curr_pos.1 + 1), Direction::Right)
        } else {
            ((curr_pos.0, curr_pos.1 - 1), Direction::Left)
        },
        Direction::Down => if infected {
            ((curr_pos.0, curr_pos.1 - 1), Direction::Left)
        } else {
            ((curr_pos.0, curr_pos.1 + 1), Direction::Right)
        },
        Direction::Left => if infected {
            ((curr_pos.0 - 1, curr_pos.1), Direction::Up)
        } else {
            ((curr_pos.0 + 1, curr_pos.1), Direction::Down)
        },
        Direction::Right => if infected {
            ((curr_pos.0 + 1, curr_pos.1), Direction::Down)
        } else {
            ((curr_pos.0 - 1, curr_pos.1), Direction::Up)
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
