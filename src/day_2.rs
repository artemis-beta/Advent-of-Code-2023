use regex::Regex;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use log;


#[derive(Eq,Hash,PartialEq)]
pub enum Color {
    Red,
    Green,
    Blue
}

pub fn game_permitted(game_input: &String, available_cubes: &HashMap<Color, i32>) -> bool {
    let game_re = match Regex::new(r"([\s\w\d,]+)") {
        Ok(r) => r,
        Err(e) => panic!("Failed to initialise regex pattern matching for set: {}", e)
    };
    let re_red = match Regex::new(r"(\d+) red") {
        Ok(r) => r,
        Err(e) => panic!("Failed to initialise regex pattern matching for red cubes: {}", e)
    };
    let re_blue = match Regex::new(r"(\d+) blue") {
        Ok(r) => r,
        Err(e) => panic!("Failed to initialise regex pattern matching for blue cubes: {}", e)
    };
    let re_green = match Regex::new(r"(\d+) green") {
        Ok(r) => r,
        Err(e) => panic!("Failed to initialise regex pattern matching for green cubes: {}", e)
    };

    let n_red_in_game = match available_cubes.get(&Color::Red) {
        Some(n) => n,
        None => &0
    };
    let n_green_in_game = match available_cubes.get(&Color::Green) {
        Some(n) => n,
        None => &0
    };
    let n_blue_in_game = match available_cubes.get(&Color::Blue) {
        Some(n) => n,
        None => &0
    };

    let re_colors = vec![re_red, re_green, re_blue];
    let n_colors = vec![*n_red_in_game, *n_green_in_game, *n_blue_in_game];

    for set in game_re.find_iter(game_input) {
        let res_string = set.as_str().to_string();

        for (capture_re, n_color) in re_colors.iter().zip(&n_colors) {
            match capture_re.captures_iter(&res_string).next() {
                Some(r) => {
                    match r.get(1) {
                        Some(g1) => match g1.as_str().parse::<i32>() {
                            Ok(n) => {
                                if n > *n_color {
                                    return false;
                                }
                            },
                            Err(e) => panic!("Failed to parse '{}': {}", g1.as_str(), e)
                        },
                        None => ()
                    }
                },
                None => ()
            };
        }
    }
    true
}

pub fn get_total_of_permitted_game_ids(game_record: &String, available_cubes: &HashMap<Color, i32>) -> Result<i32, String> {
    let in_file = match File::open(game_record) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", game_record, e))
    };
    let file_reader = BufReader::new(in_file);

    let game_id_re = match Regex::new(r"Game (\d+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern for game ID: {}", e))
    };

    let mut total = 0;

    for line in file_reader.lines() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };

        log::info!("Checking validity of game from line: {}", file_line);

        match game_id_re.captures_iter(&file_line).next() {
            Some(r) => {
                match r.get(1) {
                    Some(g1) => match g1.as_str().parse::<i32>() {
                        Ok(n) => {
                            if game_permitted(&file_line, &available_cubes) {
                                log::debug!("Game permitted, adding identifier of '{}' to total", n);
                                total += n;
                            }
                        },
                        Err(e) => return Err(format!("Failed to parse '{}': {}", g1.as_str(), e))
                    },
                    None => ()
                }
            },
            None => ()
        };
    
    }

    Ok(total)
   
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_game_permitted() {
        let mut cubes = HashMap::new();
        cubes.insert(Color::Red, 12);
        cubes.insert(Color::Green, 13);
        cubes.insert(Color::Blue, 14);

        let example_game_pass = "Game X: 7 blue, 6 green; 5 red, 9 green; 1 blue, 6 red, 5 green".to_string();
        let example_game_fail = "Game Y: 12 red, 15 green; 4 red, 6 blue, 5 green".to_string();

        assert!(game_permitted(&example_game_pass, &cubes));
        assert!(!game_permitted(&example_game_fail, &cubes));
    }

    #[test]
    fn test_total_of_passed_game_ids() {
        let mut cubes = HashMap::new();
        cubes.insert(Color::Red, 12);
        cubes.insert(Color::Green, 13);
        cubes.insert(Color::Blue, 14);

        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_2_1.dat");
        assert_eq!(get_total_of_permitted_game_ids(&test_file.to_str().unwrap().to_string(), &cubes).unwrap(), 8);
    }
}