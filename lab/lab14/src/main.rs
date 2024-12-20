#![feature(test)]
extern crate test;

use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn test_parse(b: &mut Bencher) {
        b.iter(|| {
            let map_str = std::fs::read_to_string("data.txt").unwrap();
            let map: Vec<Vec<bool>> = map_str
                .trim()
                .split('\n')
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect();
            map
        });
    }

    #[bench]
    fn test_expand(b: &mut Bencher) {
        let map_str = std::fs::read_to_string("data.txt").unwrap();
        let map: Vec<Vec<bool>> = map_str
            .trim()
            .split('\n')
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        b.iter(|| expand_map(&map));
    }

    #[bench]
    fn test_shortest_path(b: &mut Bencher) {
        let map_str = std::fs::read_to_string("data.txt").unwrap();
        let map: Vec<Vec<bool>> = map_str
            .trim()
            .split('\n')
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();

        let expanded_map = expand_map(&map);

        // Now that we have our map, we can start to solve the problem
        let map = Map::from_2d_vec(expanded_map);
        b.iter(|| map.shortest_path_sum());
    }
}

enum Cell {
    Empty,
    Island,
}

struct Map {
    cells: Vec<Cell>, // 1d structure for spatial cache locality
    width: usize,
    #[allow(dead_code)]
    height: usize,
}

impl Map {
    #[allow(dead_code)]
    fn from_str(map_str: String) -> Self {
        let width = map_str
            .trim()
            .find('\n')
            .expect("map must have at least one row");
        // let width = map_str.first().unwrap().len();
        let height = map_str.lines().count();

        let cells: Vec<Cell> = map_str
            .trim()
            .chars()
            .filter(|c| *c != '\n')
            .map(|c| if c == '#' { Cell::Island } else { Cell::Empty })
            .collect();

        Map {
            cells,
            width,
            height,
        }
    }

    fn from_2d_vec(map: Vec<Vec<bool>>) -> Self {
        let width = map.first().unwrap().len();
        let height = map.len();
        let cells: Vec<Cell> = map
            .iter()
            .flat_map(|line| {
                line.iter()
                    .map(|&c| if c { Cell::Island } else { Cell::Empty })
            })
            .collect();
        Map {
            cells,
            width,
            height,
        }
    }

    // fn get(&self, x: usize, y: usize) -> Option<&Cell> {
    //     self.cells.get(y * self.width + x)
    // }

    fn get_islands(&self) -> Vec<usize> {
        self.cells
            .iter()
            .enumerate()
            .filter_map(|(index, cell)| match cell {
                Cell::Island => Some(index),
                _ => None,
            })
            .collect()
    }
    fn shortest_path_sum(&self) -> usize {
        let islands = self.get_islands();
        let mut shortest_path_sum = 0;
        for (i, &island1) in islands.iter().enumerate() {
            let island1y = island1 / self.width;
            let island1x = island1 % self.width;

            for &next in islands.iter().skip(i) {
                let next_y = next / self.width;
                let next_x = next % self.width;
                let path_x = if island1x > next_x {
                    island1x - next_x
                } else {
                    next_x - island1x
                };
                let path_y = next_y - island1y;
                shortest_path_sum += path_x + path_y;
            }
        }
        shortest_path_sum
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // read test file
    let map_str = std::fs::read_to_string("data.txt")?;

    let map: Vec<Vec<bool>> = map_str
        .trim()
        .split('\n')
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    let expanded_map = expand_map(&map);

    // Now that we have our map, we can start to solve the problem
    let parsed_map = Map::from_2d_vec(expanded_map);

    let shortest_path = parsed_map.shortest_path_sum();
    println!("Shortest path: {}", shortest_path);
    Ok(())
}

fn expand_map(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let horizontal_expanded = expand_horizontal(map);
    expand_vertical(&horizontal_expanded)
}

fn expand_vertical(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut vertical_expanded_map: Vec<Vec<bool>> = Vec::<Vec<bool>>::new();
    map.iter().for_each(|line| {
        if line.iter().all(|x| !x) {
            // we could clone the line, but this is faster
            vertical_expanded_map.push(vec![false; line.len()]);
            vertical_expanded_map.push(vec![false; line.len()]);
        } else {
            vertical_expanded_map.push(line.clone());
        }
    });

    vertical_expanded_map
}

fn expand_horizontal(map: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut expanded_map: Vec<Vec<bool>> = Vec::<Vec<bool>>::with_capacity(map.len());
    // find all vertical lines with all false values
    let row_length = map.first().unwrap().len();

    for x in 0..row_length {
        // for each row
        let mut has_islands = false;
        for y in 0..map.len() {
            // for each point in row
            if map[y][x] {
                has_islands = true;
                break;
            }
        }
        if has_islands {
            transfer_column(&mut expanded_map, &map, x);
        } else {
            add_blank_column(&mut expanded_map);
            add_blank_column(&mut expanded_map);
        }
    }
    expanded_map
}

fn transfer_column(target: &mut Vec<Vec<bool>>, source: &Vec<Vec<bool>>, x: usize) {
    while target.len() < source.len() {
        target.push(vec![false; x]);
    }

    target.iter_mut().enumerate().for_each(|(y, line)| {
        line.push(source[y][x]);
    });
}

fn add_blank_column(map: &mut Vec<Vec<bool>>) {
    map.iter_mut().for_each(|line| {
        line.push(false);
    });
}

// fn print_map(name: &str, map: &Vec<Vec<bool>>) {
//     println!("Map: {}", name);
//     for line in map {
//         for point in line {
//             print!("{}", if *point { '#' } else { '.' });
//         }
//         println!();
//     }
// }
