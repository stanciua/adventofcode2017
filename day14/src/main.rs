#![feature(inclusive_range_syntax)]

fn get_hasher_from_input(input: &str) -> String {
    let mut lengths = input.as_bytes().iter().cloned().collect::<Vec<_>>();
    lengths.extend(vec![17u8, 31, 73, 47, 23]);

    let mut curr_pos = 0;
    let mut skip_size = 0;
    let mut sparse_hash = (0..=255).into_iter().collect::<Vec<_>>();

    for _ in 0..64 {
        run_iteration(
            sparse_hash.as_mut_slice(),
            lengths.as_slice(),
            &mut curr_pos,
            &mut skip_size,
        );
    }

    let mut dense_hash = Vec::new();
    for slice in sparse_hash.chunks(16) {
        dense_hash.push(slice.iter().fold(0, |mut acc, v| {
            acc ^= v;
            acc
        }));
    }
    dense_hash
        .iter()
        .map(|v| format!("{:02x}", v))
        .fold(String::new(), |mut acc, v| {
            acc.extend(v.chars());
            acc
        })
}
fn main() {
    let input = "flqrgnkx";
    let disk = get_disk(input, 128);
    let count = get_count(&disk);
    println!("Squares used: {:?}", count);
    println!("No of regions is: {:?}", get_no_regions(&disk));
}

fn get_count(disk: &[String]) -> usize {
    let mut total = 0;
    for row in disk {
        total += row.chars()
            .map(|c| format!("{:4b}", c.to_digit(16).unwrap()))
            .collect::<String>()
            .chars()
            .filter(|&c| c == '1')
            .count();
    }
    total
}

fn get_no_regions(disk: &[String]) -> usize {
    let mut disk_bin = Vec::new();
    for row in disk {
        let row_bin = row.chars()
            .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
            .collect::<Vec<_>>()
            .iter()
            .flat_map(|s| s.chars())
            .map(|c| {
                let d = c.to_digit(10).unwrap();
                if d == 1 {
                    -1
                } else {
                    0
                }
            })
            .collect::<Vec<_>>();
        disk_bin.push(row_bin);
    }
    // insert regions inside disk data
    let mut region = 1;
    // for i in 0..disk_bin.len() {
    //     for j in 0..disk_bin[i].len() {
    //         if disk_bin[i][j] == -1 {
    //             if let Some(r) = get_surround_region(i, j, &disk_bin) {
    //                 set_adjacent_squares_region(i, j, &mut disk_bin, r);
    //             } else {
    //                 set_adjacent_squares_region(i, j, &mut disk_bin, region);
    //                 region += 1;
    //             }
    //         }
    //     }
    // }

    // for i in 0..32 {
    //     for j in 0..32 {
    //         print!("{},", disk_bin[i][j]);
    //     }
    //     println!();
    // }
    region as usize
}
fn destination_reached(disk: &[Vec<i32>], i: usize, j: usize) -> bool {
    let mut dead_end = true;
    if i as i32 - 1 >= 0 && disk[i - 1][j] == -1 {
        dead_end = false;
    }
    if i as i32 + 1 < disk.len() as i32 && disk[i + 1][j] == -1 {
        dead_end = false;
    }
    if j as i32 - 1 >= 0 && disk[i][j - 1] == -1 {
        dead_end = false;
    }
    if j as i32 + 1 < disk.len() as i32 && disk[i][j + 1] == -1 {
        dead_end = false;
    }
    dead_end
}
fn bt(disk: &[Vec<i32>], c: (usize, usize), solution: &mut Vec<(usize, usize)>) {
    if destination_reached(disk, c.0, c.1) {
        return;
    }
    solution.push(c);
}
// If destination is reached
//     print the solution matrix
// Else
//    a) Mark current cell in solution matrix as 1.
//    b) Move forward in horizontal direction and recursively check if this
//        move leads to a solution.
//    c) If the move chosen in the above step doesn't lead to a solution
//        then move down and check if  this move leads to a solution.
//    d) If none of the above solutions work then unmark this cell as 0
//        (BACKTRACK) and return false.

// fn set_adjacent_squares_region(i: usize, j: usize, disk: &mut [Vec<i32>], region: i32) {
//     if i as i32 - 1 >= 0 && disk[i - 1][j] == -1 {
//         disk[i - 1][j] = region;
//     }
//     if i as i32 + 1 < disk.len() as i32 && disk[i + 1][j] == -1 {
//         disk[i + 1][j] = region;
//     }
//     if j as i32 - 1 >= 0 && disk[i][j - 1] == -1 {
//         disk[i][j - 1] = region;
//     }
//     if j as i32 + 1 < disk.len() as i32 && disk[i][j + 1] == -1 {
//         disk[i][j + 1] = region;
//     }
//     // and also set the current position region
//     disk[i][j] = region;
// }
// fn get_surround_region(i: usize, j: usize, disk: &[Vec<i32>]) -> Option<i32> {
//     // check for adjacent regions
//     //    |
//     // --  --
//     //    |
//     if i as i32 - 1 >= 0 && disk[i - 1][j] != -1 && disk[i - 1][j] != 0 {
//         return Some(disk[i - 1][j]);
//     }
//     if i as i32 + 1 < disk.len() as i32 && disk[i + 1][j] != -1 && disk[i + 1][j] != 0 {
//         return Some(disk[i + 1][j]);
//     }
//     if j as i32 - 1 >= 0 && disk[i][j - 1] != -1 && disk[i][j - 1] != 0 {
//         return Some(disk[i][j - 1]);
//     }
//     if j as i32 + 1 < disk.len() as i32 && disk[i][j + 1] != -1 && disk[i][j + 1] != 0 {
//         return Some(disk[i][j + 1]);
//     }
//     None
// }
fn get_disk(input: &str, disk_size: usize) -> Vec<String> {
    (0..disk_size)
        .into_iter()
        .map(|v| get_hasher_from_input(&format!("{}-{}", input, v)))
        .collect::<Vec<_>>()
}
fn run_iteration(list: &mut [u8], lengths: &[u8], curr_pos: &mut usize, skip_size: &mut usize) {
    let len = list.len();
    lengths
        .iter()
        .fold((list, curr_pos, skip_size), |state, &n| {
            let sublist = state
                .0
                .iter()
                .cycle()
                .skip(*state.1 as usize)
                .take(len)
                .cloned()
                .collect::<Vec<_>>();

            let mut rev_sublist = sublist
                .iter()
                .take(n as usize)
                .collect::<Vec<_>>()
                .into_iter()
                .cloned()
                .rev()
                .collect::<Vec<_>>();
            rev_sublist.extend(sublist.into_iter().skip(n as usize).take(len - n as usize));

            for (idx, e) in rev_sublist.into_iter().enumerate() {
                state.0[(*state.1 + idx) % len] = e;
            }

            *state.1 = (*state.1 + n as usize + *state.2) % len;
            *state.2 = (*state.2 + 1) % len;
            state
        });
}
