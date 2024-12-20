use rayon::prelude::*;
use std::error::Error;
use std::time::Instant;

const EXPANDED_SIZE: usize = 2;
// const EXPANDED_SIZE: usize = 1_000_000;

type Island = (usize, usize);

fn main() -> Result<(), Box<dyn Error>> {
    let files = [
        "test-data.txt",
        "data.txt",
        "datasets/10k.txt",
        "datasets/100k.txt",
    ];

    for file in files.iter() {
        println!();
        println!("File: {}", file);

        let start = Instant::now();
        let result = calculate_result(file)?;
        let duration = start.elapsed();

        println!("Total Time: {:?}", duration);
        println!("Result: {}", result);
    }

    Ok(())
}

// do map_str str to islands and shortest path
fn calculate_result(file: &str) -> Result<usize, Box<dyn Error>> {
    let map_str = std::fs::read_to_string(file).expect("file not found");
    let islands = str_to_islands(map_str);
    Ok(shortest_path_sum(islands))
}

fn str_to_islands(map_str: String) -> Vec<Island> {
    let width = map_str
        .trim()
        .find('\n')
        .expect("map must have at least one row");
    let height = map_str.lines().count();

    let mut empty_row: Vec<bool> = vec![true; width];
    let mut empty_column: Vec<bool> = vec![true; height];
    let mut islands: Vec<Island> = Vec::new();

    for (y, line) in map_str.lines().enumerate() {
        for (x, c) in line.bytes().enumerate() {
            if c == b'#' {
                empty_row[x] = false;
                empty_column[y] = false;
            }
        }
    }
    let mut y_expanded = 0;
    for (y, line) in map_str.lines().enumerate() {
        let mut x_expanded = 0;
        y_expanded += if empty_column[y] { EXPANDED_SIZE } else { 1 };
        for (x, c) in line.bytes().enumerate() {
            // if current pos is island
            x_expanded += if empty_row[x] { EXPANDED_SIZE } else { 1 };
            if c == b'#' {
                islands.push((x_expanded, y_expanded));
            }
        }
    }
    islands
}

// 8x faster on my machine by using rayon for paralleliztion.
// because of the parallelization overhead, this only saves time on larger datasets.
fn shortest_path_sum(islands: Vec<Island>) -> usize {
    let sum = islands
        .par_iter()
        .enumerate()
        .map(|(i, &isl)| {
            islands
                .iter()
                .skip(i)
                .map(|&snd_isl| manhattan_distance(isl.0, isl.1, snd_isl.0, snd_isl.1))
                .sum::<usize>()
        })
        .sum();
    sum
}

fn manhattan_distance(x1: usize, y1: usize, x2: usize, y2: usize) -> usize {
    // check which is bigger to avoid negative values
    let x = if x1 > x2 { x1 - x2 } else { x2 - x1 };
    // 40% faster than the above check, but might cut off large values:
    // let x = (x1 as isize - x2 as isize).abs() as usize;
    x + (y2 - y1)
}

// shortest path old code:
//
// Non optimized version of loop
// let mut sum = 0;
// for (i, &main_island) in islands.iter().enumerate() {
//     for &other_island in islands.iter().skip(i) {
//         sum += manhattan_distance(main_island, other_island);
//     }
// }

// 30% faster than the above loop
// Optimized loop. Since Rust is bad at exterior optimizing,
// I directly used the for_each method instead of a for-loop. It does the same thing,
// but its easier for the compiler to optimize.
// islands.iter().enumerate().for_each(|(i, &main_island)| {
//     islands.iter().skip(i).for_each(|&other_island| {
//         sum += manhattan_distance(main_island, other_island);
//     });
// });

// manhattan_distance old code:
//
// since y2 always will be bigger, we can avoid the check.
// This saved about 40% ofthe time.
// let y = if y1 > y2 { y1 - y2 } else { y2 - y1 };
