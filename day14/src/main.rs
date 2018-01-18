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
    let input = "hxtvlmkl";
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
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<_>>();
        disk_bin.push(row_bin);
    }

    let mut used_squares = Vec::new();
    for i in 0..disk_bin.len() {
        for j in 0..disk_bin[i].len() {
            if disk_bin[i][j] == 1 {
                used_squares.push((i, j));
            }
        }
    }

    let mut no_region = 0;
    for i in 0..disk_bin.len() {
        for j in 0..disk_bin[i].len() {
            if disk_bin[i][j] == 1 {
                no_region += 1;
                reset_neighbors(i, j, disk_bin.as_mut_slice());
            }
        }
    }

    no_region as usize
}

fn reset_neighbors(i: usize, j: usize, disk: &mut [Vec<u32>]) {
    if disk[i][j] == 1 {
        disk[i][j] = 0;
    } else {
        return;
    }
    // go right
    if j + 1 < disk.len() {
        reset_neighbors(i, j + 1, disk);
    }
    // go left
    if j as i32 - 1 >= 0 {
        reset_neighbors(i, j - 1, disk);
    }
    // go up
    if i as i32 - 1 >= 0 {
        reset_neighbors(i - 1, j, disk);
    }
    // go down
    if i + 1 < disk.len() {
        reset_neighbors(i + 1, j, disk);
    }
}

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
