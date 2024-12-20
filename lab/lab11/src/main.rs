use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("data2.txt")?;
    let reader = BufReader::new(file);

    let mut sum: u16 = 0;
    for line in reader.lines() {
        let numbers = extract_numbers(&line?);
        let Some(first) = numbers.first() else {
            continue;
        };
        let Some(second) = numbers.last() else {
            continue;
        };
        let combined = (first * 10) + second;
        sum += combined;
    }

    println!("Answer: {}", sum % 360);

    Ok(())
}

#[derive(Debug)]
struct Number {
    number: u16,
    index: u16,
}

fn extract_numbers(line: &str) -> Vec<u16> {
    // generate string to int map
    let number_strings: Vec<&str> = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut result_vec = Vec::<Number>::new();

    // add digits as numbers
    for (index, c) in line.chars().enumerate() {
        match c.to_digit(10) {
            Some(value) => result_vec.push(Number {
                number: value as u16,
                index: index as u16,
            }),
            None => (),
        }
    }
    // add strings as numbers
    for (number_int, number_string) in number_strings.iter().enumerate() {
        match line.find(number_string) {
            Some(i) => result_vec.push(Number {
                number: (number_int + 1) as u16,
                index: i as u16,
            }),
            None => (),
        }
    }

    // sort result_vec by index and unpack to u16
    result_vec.sort_by_key(|x| x.index);
    result_vec.iter().map(|x| x.number).collect()
}
