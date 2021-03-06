use std::fs::File;
use std::io::Read;

fn main() {
    let path = "input.txt";
    let mut input = File::open(path).expect("Unable to open file!");
    let mut input_txt = String::new();
    match input.read_to_string(&mut input_txt) {
        Err(_) => return,
        Ok(n) => println!("Read {} bytes", n),
    }

    let hexes = input_txt.split(',').collect::<Vec<_>>();
    println!(
        "The minimum number of steps is: {:?}",
        get_no_of_steps(&hexes)
    );
    println!(
        "The maximum distance is: {:?}",
        get_the_max_distance(&hexes)
    );
}

fn get_new_coordinate(curr_coordinate: (i32, i32, i32), hex: &str) -> (i32, i32, i32) {
    match hex {
        "n" => (
            curr_coordinate.0,
            curr_coordinate.1 + 1,
            curr_coordinate.2 - 1,
        ),
        "ne" => (
            curr_coordinate.0 + 1,
            curr_coordinate.1,
            curr_coordinate.2 - 1,
        ),
        "se" => (
            curr_coordinate.0 + 1,
            curr_coordinate.1 - 1,
            curr_coordinate.2,
        ),
        "s" => (
            curr_coordinate.0,
            curr_coordinate.1 - 1,
            curr_coordinate.2 + 1,
        ),
        "sw" => (
            curr_coordinate.0 - 1,
            curr_coordinate.1,
            curr_coordinate.2 + 1,
        ),
        "nw" => (
            curr_coordinate.0 - 1,
            curr_coordinate.1 + 1,
            curr_coordinate.2,
        ),
        _ => panic!("invalid input received"),
    }
}

fn get_the_max_distance(hexes: &[&str]) -> i32 {
    let start_coordinate = (0i32, 0i32, 0i32);
    let mut curr_coordinate = start_coordinate;

    let mut distances = Vec::new();
    for hex in hexes {
        curr_coordinate = get_new_coordinate(curr_coordinate, hex);
        distances.push(
            ((start_coordinate.0 - curr_coordinate.0).abs()
                + (start_coordinate.1 - curr_coordinate.1).abs()
                + (start_coordinate.2 - curr_coordinate.2).abs()) / 2,
        );
    }
    *distances.iter().max().unwrap()
}
fn get_no_of_steps(hexes: &[&str]) -> i32 {
    let start_coordinate = (0i32, 0i32, 0i32);
    let mut curr_coordinate = start_coordinate;

    for hex in hexes {
        curr_coordinate = get_new_coordinate(curr_coordinate, hex);
    }

    ((start_coordinate.0 - curr_coordinate.0).abs() + (start_coordinate.1 - curr_coordinate.1).abs()
        + (start_coordinate.2 - curr_coordinate.2).abs()) / 2
}
