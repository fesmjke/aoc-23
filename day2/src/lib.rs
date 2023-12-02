use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

use template::read_file;

fn check_cubes(chars: &Vec<char>, cubes: &Vec<(usize, &str)>, limit: u32) -> bool {
    let mut result_in_line = vec![];

    for (index, _) in cubes.iter() {
        let mut number_of_cubes = String::new();

        let mut temp = *index - 2; // offset

        while temp != 0 {
            if chars[temp].is_whitespace() {
                break;
            }

            if chars[temp].is_digit(10) {
                number_of_cubes.insert(0, chars[temp]);
            }

            temp -= 1;
        }

        let number_of_cubes = number_of_cubes
            .parse::<u32>()
            .expect("Unable to parse number!");

        if number_of_cubes > limit {
            result_in_line.push(false);
        }
    }

    result_in_line.into_iter().filter(|x| *x == false).count() == 0
}

fn count_cubes(chars: &Vec<char>, cubes: &Vec<(usize, &str)>) -> u32 {
    let mut maximum_number_of_cubes = 0;

    for (index, _) in cubes.iter() {
        let mut number_of_cubes = String::new();

        let mut temp = *index - 2; // offset

        while temp != 0 {
            if chars[temp].is_whitespace() {
                break;
            }

            if chars[temp].is_digit(10) {
                number_of_cubes.insert(0, chars[temp]);
            }

            temp -= 1;
        }

        let number_of_cubes = number_of_cubes
            .parse::<u32>()
            .expect("Unable to parse number!");

        match number_of_cubes.cmp(&maximum_number_of_cubes) {
            Ordering::Less => {}
            Ordering::Equal => {}
            Ordering::Greater => maximum_number_of_cubes = number_of_cubes,
        }
    }

    maximum_number_of_cubes
}

pub fn cube_conundrum(path: &str) -> u32 {
    let lines = read_file!(path);

    let mut power_sets = 0;

    for (_, line) in lines.into_iter().enumerate() {
        match line {
            Ok(line) => {
                let red: Vec<_> = line.match_indices("red").collect();
                let green: Vec<_> = line.match_indices("green").collect();
                let blue: Vec<_> = line.match_indices("blue").collect();

                let line_chars = line.chars().collect::<Vec<_>>();

                let mut temp = 1;

                temp *= count_cubes(&line_chars, &red);
                temp *= count_cubes(&line_chars, &green);
                temp *= count_cubes(&line_chars, &blue);

                power_sets += temp;
            }
            Err(_) => {}
        }
    }

    power_sets
}
