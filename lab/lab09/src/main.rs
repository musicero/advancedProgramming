use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Element {
    uid: u32,
    #[serde(alias = "greenstoneWeight")]
    greenstone_weight: u32,
    #[serde(alias = "waterGuardian")]
    water_guardian: String,
    #[serde(alias = "vulcanicCave")]
    vulcanic_cave: u32,
    #[serde(alias = "skyRiddle")]
    sky_riddle: String,
}

fn main() {
    // reading to string first instead of using buffer
    // is faster because of the nature of the JSON format.
    let content = read_file("test-data.json");
    // no need to continue if json data is corrupt
    let mut elements: Vec<Element> = serde_json::from_str(&content).expect("Could not parse JSON");
    elements.sort_by_key(|a| a.uid);

    let mut output = String::new();
    for element in elements {
        output += &format_element(&element);
    }
    match fs::write("answers.txt", output) {
        Ok(_) => (),
        Err(e) => println!("Could not write to file: {}", e),
    }
}

fn format_element(elm: &Element) -> String {
    let mut errors = Vec::new();

    let greenstone_errors = validate_greenstone(elm.greenstone_weight);
    errors.extend(prefix_errors("Greenstone weight", greenstone_errors));

    let water_errors = validate_water(&elm.water_guardian);
    errors.extend(prefix_errors("Water guardian name", water_errors));

    let vulcanic_errors = validate_vulcanic(elm.vulcanic_cave);
    errors.extend(prefix_errors("Vulcanic cave temperature", vulcanic_errors));

    let sky_errors = validate_sky(&elm.sky_riddle);
    errors.extend(prefix_errors("Sky riddle", sky_errors));

    if errors.len() < 1 {
        return "Valid".to_string();
    }

    format!("{}: {}", elm.uid, errors.join(", "))
}

fn prefix_errors(prefix: &str, errors: Vec<String>) -> Vec<String> {
    let mut formatted_errors = Vec::new();
    format!("{}: ", prefix);

    for error in errors.iter() {
        formatted_errors.push(format!("{}: {}", prefix, error));
    }

    formatted_errors
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => content,
        Err(_) => panic!("Couldnt open file"),
    }
}

fn validate_greenstone(weight: u32) -> Vec<String> {
    let mut err = Vec::new();
    if !in_range(weight, 13, 113) {
        err.push("outside of range".to_string());
    }

    if !is_even(weight) {
        err.push("not even".to_string());
    }

    err
}

fn validate_water(guardian: &String) -> Vec<String> {
    let mut err = Vec::new();

    if !in_range(guardian.len() as u32, 3, 15) {
        err.push("outside of range".to_string());
    }

    match guardian.matches("a").count() {
        x if x < 2 => err.push("missing 'a'".to_string()),
        x if x > 2 => err.push("excessive 'a'".to_string()),
        _ => (),
    }

    // length already checked, should not panic
    if !guardian.chars().next().unwrap().is_uppercase() {
        err.push("not capitalized".to_string());
    }

    err
}

fn validate_vulcanic(cave: u32) -> Vec<String> {
    let mut err = Vec::new();

    if !in_range(cave, 400, 700) {
        err.push("outside the range".to_string());
    }

    if !is_even(cave) {
        err.push("not even".to_string());
    }

    err
}

fn validate_sky(riddle: &String) -> Vec<String> {
    let mut err = Vec::new();
    match riddle.chars().next() {
        Some(c) => match c.is_uppercase() {
            false => err.push("not capitalized".to_string()),
            _ => (),
        },
        None => err.push("not capitalized".to_string()),
    }

    if !riddle.ends_with(".") {
        err.push("missing dot".to_string());
    }

    // considering y as a vowel
    let count = riddle
        .chars()
        .filter(|c| c.is_lowercase() && "aeiouy".contains(*c))
        .count();
    if !is_even(count as u32) {
        err.push("not even".to_string());
    }

    err
}

fn is_even(num: u32) -> bool {
    num % 2 == 0
}

fn in_range(num: u32, min: u32, max: u32) -> bool {
    min <= num && num <= max
}
