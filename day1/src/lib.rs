use std::fs::File;
use std::io::{BufRead, BufReader};

use template::read_file;

fn literal_to_digit(raw: &str) -> u32 {
    match raw {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => 0,
    }
}

pub fn calibration(path: &str) -> u32 {
    let lines = read_file!(path);

    let mut calibration_value = 0;

    for line in lines.into_iter() {
        match line {
            Ok(line) => {
                let mut number = String::new();
                for chr in line.chars().into_iter() {
                    if chr.is_digit(10) {
                        if number.len() == 2 {
                            number.pop();
                            number.push(chr);
                            continue;
                        }

                        number.push(chr);
                    }
                }

                calibration_value += match number.len() {
                    0 => 0,
                    1 => {
                        let base = number.parse::<u32>().expect("Unable to parse number!");
                        (base * 10) + base
                    }
                    2 => number.parse::<u32>().expect("Unable to parse number!"),
                    _ => 0,
                }
            }
            Err(_) => {}
        }
    }

    calibration_value
}
