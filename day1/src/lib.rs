use std::fs::File;
use std::io::{BufRead, BufReader};

use template::read_file;

fn literal_to_digit(raw: &str) -> char {
    match raw {
        "one" => '1',
        "two" => '2',
        "three" => '3',
        "four" => '4',
        "five" => '5',
        "six" => '6',
        "seven" => '7',
        "eight" => '8',
        "nine" => '9',
        _ => '0',
    }
}

pub fn calibration(path: &str) -> u32 {
    let lines = read_file!(path);

    let mut calibration_value = 0;

    let digits = vec![
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    for line in lines.into_iter() {
        match line {
            Ok(mut line) => {
                let mut number = vec![];

                let mut digits_indexes = vec![];
                let temp = line.clone();

                for digit in digits.iter() {
                    let v: Vec<_> = temp.match_indices(digit).collect();

                    digits_indexes.push(v);
                }

                let mut digits_indexes = digits_indexes.into_iter().flatten().collect::<Vec<_>>();

                digits_indexes.sort_by_key(|x| x.0);

                let mut offset = 0;
                for (index, digit) in digits_indexes.into_iter() {
                    line.insert(index + offset, literal_to_digit(digit));
                    offset += 1;
                }

                for chr in line.chars().into_iter() {
                    if chr.is_digit(10) {
                        number.push(chr.to_digit(10).expect("Unable to convert char to digit!"));
                    }
                }

                calibration_value += match number.len() {
                    0 => 0,
                    1 => {
                        println!("{}", ((number[0] * 10) + number[0]));
                        (number[0] * 10) + number[0]
                    }
                    _ => {
                        println!("{}", (number[0] * 10) + number[number.len() - 1]);
                        (number[0] * 10) + number[number.len() - 1]
                    }
                }
            }
            Err(_) => {}
        }
    }

    calibration_value
}
