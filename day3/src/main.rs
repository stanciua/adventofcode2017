fn main() {
    let size = get_grid_size_from_input(289326);
    let from_pos = get_input_pos_in_the_grid(size, 289326);
    let to_pos = (size / 2, size / 2);
    let manhattan_distance = (from_pos.0 - to_pos.0).abs() + (from_pos.1 - to_pos.1).abs();
    println!("manhattan_distance: {:?}", manhattan_distance);
    println!(
        "The value larger than input is: {:?}",
        get_largest_num_bigger_than_input(size, 289326)
    );
}

fn get_grid_size_from_input(input: i32) -> i32 {
    let mut last_size = 0;
    for i in 0..input {
        if i <= (input as f64).sqrt().ceil() as i32 {
            last_size = i;
        }
    }

    if last_size % 2 == 0 {
        last_size + 1
    } else {
        last_size
    }
}

fn get_input_pos_in_the_grid(grid_size: i32, input: i32) -> (i32, i32) {
    let mut grid = vec![vec![0; grid_size as usize]; grid_size as usize];
    let mut curr_pos = (grid_size - 1, grid_size - 1);
    let stop_pos = (grid_size / 2, grid_size / 2);
    let mut curr_num = grid_size * grid_size;
    while curr_pos != stop_pos {
        // first try to go left
        while curr_pos.1 - 1 >= 0 && grid[curr_pos.0 as usize][(curr_pos.1 - 1) as usize] == 0 {
            // we just need to get to the input value and take its position
            if curr_num == input {
                return curr_pos;
            }
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0, curr_pos.1 - 1);
        }
        // next, try to go up
        while curr_pos.0 - 1 >= 0 && grid[(curr_pos.0 - 1) as usize][curr_pos.1 as usize] == 0 {
            if curr_num == input {
                return curr_pos;
            }

            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0 - 1, curr_pos.1);
        }
        // next, try to go right
        while curr_pos.1 + 1 < grid_size
            && grid[curr_pos.0 as usize][(curr_pos.1 + 1) as usize] == 0
        {
            if curr_num == input {
                return curr_pos;
            }

            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0, curr_pos.1 + 1);
        }
        // next, try to go down
        while curr_pos.0 + 1 < grid_size
            && grid[(curr_pos.0 + 1) as usize][curr_pos.1 as usize] == 0
        {
            if curr_num == input {
                return curr_pos;
            }

            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0 + 1, curr_pos.1);
        }

        // termination when we arrive at coordinates (grid_size / 2, grid_size / 2)
        if curr_pos == stop_pos {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            break;
        }
    }

    curr_pos
}

fn calculate_adjacent_squares_sum(grid: &[Vec<i32>], pos: (i32, i32)) -> i32 {
    let mut sum = 0;
    let grid_size = grid.len();
    // try up
    if pos.0 - 1 >= 0 && grid[(pos.0 - 1) as usize][pos.1 as usize] != 0 {
        sum += grid[(pos.0 - 1) as usize][pos.1 as usize];
    }
    // try down
    if (pos.0 + 1) as usize <= grid_size - 1 && grid[(pos.0 + 1) as usize][pos.1 as usize] != 0 {
        sum += grid[(pos.0 + 1) as usize][pos.1 as usize];
    }
    // try left
    if pos.1 - 1 >= 0 && grid[pos.0 as usize][(pos.1 - 1) as usize] != 0 {
        sum += grid[pos.0 as usize][(pos.1 - 1) as usize];
    }
    // try right
    if (pos.1 + 1) as usize <= grid_size - 1 && grid[pos.0 as usize][(pos.1 + 1) as usize] != 0 {
        sum += grid[pos.0 as usize][(pos.1 + 1) as usize];
    }
    // try up-left
    if pos.0 - 1 >= 0 && pos.1 - 1 >= 0 && grid[(pos.0 - 1) as usize][(pos.1 - 1) as usize] != 0 {
        sum += grid[(pos.0 - 1) as usize][(pos.1 - 1) as usize];
    }
    // try up-right
    if pos.0 - 1 >= 0 && (pos.1 + 1) as usize <= grid_size - 1
        && grid[(pos.0 - 1) as usize][(pos.1 + 1) as usize] != 0
    {
        sum += grid[(pos.0 - 1) as usize][(pos.1 + 1) as usize];
    }
    // try down-left
    if (pos.0 + 1) as usize <= grid_size - 1 && pos.1 - 1 >= 0
        && grid[(pos.0 + 1) as usize][(pos.1 - 1) as usize] != 0
    {
        sum += grid[(pos.0 + 1) as usize][(pos.1 - 1) as usize];
    }
    // try down-right
    if (pos.0 + 1) as usize <= grid_size - 1 && (pos.1 + 1) as usize <= grid_size - 1
        && grid[(pos.0 + 1) as usize][(pos.1 + 1) as usize] != 0
    {
        sum += grid[(pos.0 + 1) as usize][(pos.1 + 1) as usize];
    }
    sum
}

fn get_largest_num_bigger_than_input(grid_size: i32, input: i32) -> i32 {
    let mut grid = vec![vec![0; grid_size as usize]; grid_size as usize];
    let path = get_spiral_path(grid_size);
    // starting value is 1
    grid[path[0].0 as usize][path[0].1 as usize] = 1;
    for pos in path.iter().skip(1) {
        let sum = calculate_adjacent_squares_sum(&grid, *pos);
        if sum > input {
            return sum;
        }
        grid[pos.0 as usize][pos.1 as usize] = sum;
    }
    0
}
fn get_spiral_path(grid_size: i32) -> Vec<(i32, i32)> {
    let mut grid = vec![vec![0; grid_size as usize]; grid_size as usize];
    let mut path = Vec::new();
    let mut curr_pos = (grid_size - 1, grid_size - 1);
    path.push(curr_pos);
    let stop_pos = (grid_size / 2, grid_size / 2);
    let mut curr_num = grid_size * grid_size;
    while curr_pos != stop_pos {
        // first try to go left
        while curr_pos.1 - 1 >= 0 && grid[curr_pos.0 as usize][(curr_pos.1 - 1) as usize] == 0 {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0, curr_pos.1 - 1);
            path.push(curr_pos);
        }
        // next, try to go up
        while curr_pos.0 - 1 >= 0 && grid[(curr_pos.0 - 1) as usize][curr_pos.1 as usize] == 0 {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0 - 1, curr_pos.1);
            path.push(curr_pos);
        }
        // next, try to go right
        while curr_pos.1 + 1 < grid_size
            && grid[curr_pos.0 as usize][(curr_pos.1 + 1) as usize] == 0
        {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0, curr_pos.1 + 1);
            path.push(curr_pos);
        }
        // next, try to go down
        while curr_pos.0 + 1 < grid_size
            && grid[(curr_pos.0 + 1) as usize][curr_pos.1 as usize] == 0
        {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            curr_num -= 1;
            curr_pos = (curr_pos.0 + 1, curr_pos.1);
            path.push(curr_pos);
        }

        // termination when we arrive at coordinates (grid_size / 2, grid_size / 2)
        if curr_pos == stop_pos {
            grid[curr_pos.0 as usize][curr_pos.1 as usize] = curr_num;
            break;
        }
    }

    path.into_iter().rev().collect::<Vec<_>>()
}
