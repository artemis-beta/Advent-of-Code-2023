/*                        ADVENT OF CODE DAY 2

A color cube game is defined as taking a handful of cubes from a bag and determining based
on this information the total contents of the bag.

For this exercise, a collection of games are defined in the form:

```
Game X: R0 red, G0 green, B0 blue;...;RN red GN green, BN blue
```

where each semicolon separation represents a cube set.

The elf hosting the game asks how many of the games defined would be possible when the
bag contents is: 12 red, 13 green and 14 blue cubes.

Further to this, the "power" of a game is defined as the product of the maximum number
of cubes per color within a game across all sets within that game, i.e.:

P(R,G,B) = Max(Ri)*Max(Gi)*Max(Bi)

@author : K. Zarebski
@date : last modified 2023-12-03

*/

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
    /* Determine whether the given game is possible with the available cubes.

    Given a set of cubes, read in the string defining a single game of cube sets and determine
    if the game is possible (i.e. there are enough cubes of each color to represent it)

    # Arguments

    * `game_input` - the string from a game session file defining a single game
    * `available_cubes` - a hashmap containing the number of cubes of each color available

    # Examples

    ```
    let mut cubes = HashMap::new();
    cubes.insert(Color::Red, 12);
    cubes.insert(Color::Green, 13);
    cubes.insert(Color::Blue, 14);

    let example_game = "Game X: 7 blue, 6 green; 5 red, 9 green; 1 blue, 6 red, 5 green".to_string();

    assert!(game_permitted(&example_game_pass, &cubes));
    ```  

    */
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

pub fn game_power(game_input: &String) -> Result<i32, String> {
    /* Calculate the game power for the given game input.

    Calculates the power of a game consisting of N sets of colored cubes as:

    P(R,G,B) = Max(Ri)*Max(Gi)*Max(Bi)


    # Arguments

    * `game_input` - the string from a game session file defining a single game


    # Examples

    let example_game = "Game X: 7 blue, 6 green; 5 red, 9 green; 1 blue, 6 red, 5 green".to_string();

    game_power(&example_game).unwrap();
    ```
     */
    let game_re = match Regex::new(r"([\s\w\d,]+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern matching for set: {}", e))
    };
    let re_red = match Regex::new(r"(\d+) red") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern matching for red cubes: {}", e))
    };
    let re_blue = match Regex::new(r"(\d+) blue") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern matching for blue cubes: {}", e))
    };
    let re_green = match Regex::new(r"(\d+) green") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern matching for green cubes: {}", e))
    };

    let re_colors = vec![re_red, re_green, re_blue];
    let mut max_counts = vec![0, 0, 0];

    for set in game_re.find_iter(game_input) {
        let res_string = set.as_str().to_string();

        for (i, capture_re) in re_colors.iter().enumerate() {
            match capture_re.captures_iter(&res_string).next() {
                Some(r) => {
                    match r.get(1) {
                        Some(g1) => match g1.as_str().parse::<i32>() {
                            Ok(n) => {
                                max_counts[i] = if n > max_counts[i] {n} else {max_counts[i]};
                            },
                            Err(e) => return Err(format!("Failed to parse '{}': {}", g1.as_str(), e))
                        },
                        None => ()
                    }
                },
                None => ()
            };
        }
    }
    Ok(max_counts.iter().fold(1, |a1, &a2| a1 * a2))
}

pub fn get_total_of_permitted_game_ids(game_record: &String, available_cubes: &HashMap<Color, i32>) -> Result<i32, String> {
    /* Get the total of all permitted game identifiers.

    For a given input file containing definitions of multiple game rounds, return the total defined as the addition
    of all identifiers for games which are possible with the specified set of color cubes. A game is defined as:

    ```
    Game X: R0 red, G0 green, B0 blue;...;RN red GN green, BN blue
    ```

    # Arguments

    * `game_record` - a file containing lines defining games with N sets of cubes.
    * `available_cubes` - a hashmap defining how many of each color of cube is available.

    
    # Examples

    ```
    let mut cubes = HashMap::new();
    cubes.insert(Color::Red, 12);
    cubes.insert(Color::Green, 13);
    cubes.insert(Color::Blue, 14);

    match simple_logger::init_with_env() {
        Ok(l) => l,
        Err(_) => ()
    };

    get_total_of_permitted_game_ids(&"/path/to/file".to_string(), &cubes).unwrap();
    ```

    */
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

pub fn get_total_game_power(game_record: &String) -> Result<i32, String> {
    /* Find the total of all game powers

    Adds all game powers for each game defined within the specified file


    # Arguments

    * `game_record` - a file containing lines defining games with N sets of cubes.


    # Examples

    ```
    let total_game_power = match aoc23::day_2::get_total_game_power(&"/path/to/file".to_string()) {
        Ok(t) => t,
        Err(e) => panic!("{}", e)
    };
    ```

    */
    let in_file = match File::open(game_record) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", game_record, e))
    };
    let file_reader = BufReader::new(in_file);

    let mut total = 0;

    for line in file_reader.lines() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };

        log::info!("Checking validity of game from line: {}", file_line);

        total += game_power(&file_line)?;
    
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
    fn test_game_power() {

        let example_game = "Game X: 7 blue, 6 green; 5 red, 9 green; 1 blue, 6 red, 5 green".to_string();

        assert_eq!(game_power(&example_game).unwrap(), 378);
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
        test_file.push("data/test/day_2.dat");
        assert_eq!(get_total_of_permitted_game_ids(&test_file.to_str().unwrap().to_string(), &cubes).unwrap(), 8);
    }
}