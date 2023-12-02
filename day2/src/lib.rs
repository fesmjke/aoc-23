use std::fs::File;
use std::io::{BufRead, BufReader};

use template::read_file;

fn count_cubes(chars: &Vec<char>, cubes: &Vec<(usize, &str)>, limit: u32) -> bool {
    let mut temp = 0;

    let mut number_of_cubes = String::new();
    let mut result_in_line = vec![];

    for (index, _) in cubes.iter() {
        temp = *index - 2; // offset

        while temp != 0 {
            // println!("{:?}", chars[temp]);

            if chars[temp].is_whitespace() {
                break;
            }

            if chars[temp].is_digit(10) {
                number_of_cubes.insert(0, chars[temp]);
            }

            temp -= 1;
        }

        if number_of_cubes
            .parse::<u32>()
            .expect("Unable to parse number!")
            > limit
        {
            result_in_line.push(false);
        }

        result_in_line.push(true);

        number_of_cubes.clear();
    }

    result_in_line.into_iter().filter(|x| *x == false).count() == 0
}

pub fn cube_conundrum(path: &str) -> usize {
    let lines = read_file!(path);

    let mut game_ids = 0;

    for (game_id, line) in lines.into_iter().enumerate() {
        match line {
            Ok(line) => {
                let red: Vec<_> = line.match_indices("red").collect();
                let green: Vec<_> = line.match_indices("green").collect();
                let blue: Vec<_> = line.match_indices("blue").collect();

                let line_chars = line.chars().collect::<Vec<_>>();

                if count_cubes(&line_chars, &red, 12)
                    && count_cubes(&line_chars, &green, 13)
                    && count_cubes(&line_chars, &blue, 14)
                {
                    game_ids += game_id + 1
                }
            }
            Err(_) => {}
        }
    }

    game_ids
}
