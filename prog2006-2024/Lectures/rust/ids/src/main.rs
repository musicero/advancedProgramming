use rand::Rng;
use std::collections::HashMap;

// DATA_SIZE - The size of the data array in the Point struct
// used to demonstrate the impact of data localatity on performance
const DATA_SIZE: usize = 64*64*16;
const HOW_MANY: usize = 100000;

#[derive(Debug)]
struct ID {
    id: String,
    count: usize,
}

// check_occurrences - (unoptimized) Function checking for reoccurrences of a number in the vector of created IDs.
// Returns the number of repeats and the repeated value (if there was one) or None if not
pub fn check_occurrences<'a>(v: &[&'a str]) -> Option<(&'a str, usize)> {
    // Creating a hashmap
    let mut m = HashMap::new();
    // Storing IDs from the input vector in the HashMap
    v.iter().for_each(|x| *m.entry(*x).or_default() += 1);

    // Finds the ID with the highest number of reoccurrences
    m.into_iter().max_by_key(|(_, v)| *v)
}

fn passing_id() -> Box<ID> {
    Box::new(ID {
        id: "ab".to_string(),
        count: 3,
    })
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
    data: [u8; DATA_SIZE],
}

fn add(p1: &Point, p2: &Point) -> Point {
    Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
        data: p1.data,
    }
}

fn generate_points(n: usize) -> Vec<Point> {
    let mut points = Vec::new();
    // This loop will generate n random points
    // and store them in the vector
    (0..n).for_each(|_| {
        // create two random points from a given range
        // x and y are drawn from the random number generator
        let x = rand::thread_rng().gen_range(0..100);
        let y = rand::thread_rng().gen_range(0..100);
        // or
        // Which one is faster? 
        // let y = rand::random::<i32>() % 100;
        points.push(Point {
            x,
            y,
            data: [0; DATA_SIZE],
        })});

    points
}

// calculate_distance between two Points
fn calculate_distance(p1: &Point, p2: &Point) -> f64 {
    let x = (p1.x - p2.x).pow(2) as f64;
    let y = (p1.y - p2.y).pow(2) as f64;
    (x + y).sqrt()
}

// Note, that we are using a reference to a vector of Points
// even though Clippy is unhappy about it, and suggests using a slice
// Why using a reference to a vector in this specific case is justified?
#[allow(clippy::ptr_arg)]
fn calculate_distances(points: &Vec<Point>) -> Vec<f64> {
    // calculate the distances between all points
    // using iterators
    points
        .windows(2)
        .map(|w| calculate_distance(&w[0], &w[1]))
        .collect()
}

#[allow(dead_code)]
fn demo() {
    let input = ["ab", "b", "ab", "d", "a", "ab", "cd", "cd"];
    // convert the array to a vector
    let res = check_occurrences(&input);

    println!("Number of repeats: {:?}\n", res);

    let id = passing_id();
    println!("ID: {}, Count: {}", id.id, id.count);

    let p1 = Point {
        x: 1,
        y: 0,
        data: [0; DATA_SIZE],
    };
    let p2 = Point {
        x: 2,
        y: 3,
        data: [0; DATA_SIZE],
    };
    let p3 = add(&p1, &p2);
    println!("Point: {:?}", p3);
}

// simple function to add two numbers given as options
// without using the unwrap method, and without using match
// the function returns None if any of the numbers is None
// let's allow this function to be unused
#[allow(dead_code)]
fn add_options(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    //a.and_then(|x| b.map(|y| x + y))
    Some(a? + b?)
}

// We are allowing wrapping literal just for DEMO purposes here
#[allow(clippy::unnecessary_literal_unwrap)]
fn main() {
    let p = Option::Some(5);
    /* typical match or if-based code block
    let x = match p {
        Some(x) => x,
        None => 0,
    };
    */
    // using the unwrap_or method instead of the above match
    let x = p.unwrap_or(0);
    println!("Value: {}", x);

    // start the timer
    let start = std::time::Instant::now();
    let data = generate_points(HOW_MANY);
    // stop the timer
    let elapsed = start.elapsed();
    println!("Time to generate data: {:?}", elapsed);
    let start = std::time::Instant::now();
    let mut sum: f64 = 0.0;
    for _i in 0..100 {
        let distances = calculate_distances(&data);
        sum += distances[0];
    }
    let elapsed = start.elapsed();
    println!("Time to calculate distances: {:?}", elapsed);
    println!("Sum: {}", sum);
}