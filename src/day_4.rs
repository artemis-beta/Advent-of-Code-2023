/*                        ADVENT OF CODE DAY 4

A set of scratchcards is represented on an input file alongside
numbers to be matched for scoring. In the first part of this exercise
for each game the score is given by:

S(N) = (N-1)**2 || 1

In the second part, the number of matches represents the number of
duplicate cards won, where these duplicates are given by:

F(Game_ID, Matches) = [Game_ID + 1 .. Game_ID + Matches + 1]

The cards won are themselves then scored and so on with the total
of all tickets (including the initial set) being calculated.

@author : K. Zarebski
@date : last modified 2023-12-04

*/

use regex::Regex;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_scratchcard_score<F: Fn(i32, i32) -> i32>(scratchcard_data: &String, scoring: F) -> Result<i32, String> {
    /* For a given set of scratchcards find the total score using the given scoring function.

    Given a function representing the incrementation of score for each matched value calculate
    the total score for a given scratchcard.

    Scratchcard data is in the form:

    Card X: N1, .., Ni | M1, .., Mi

    Where Ni are the winning numbers and Mi the player's numbers. 

    # Arguments

    * `scratchcard_data` - a string representing the data for a single scratchcard.
    * `scoring` - a lambda/function for scoring, the function takes the initial score and the matched value and returns the new total

    # Returns

    The total score of the game

    # Example

    ```
    let scratchcard_data = "Card 1: 1 23 65 323 | 1 323".to_string();
    let scorer = |total, _| return if total < 1 {1} else {total * 2};
    let score = get_scratchcard_score(&scratchcard_data, &scorer).unwrap();
    ```

    */
    log::debug!("Reading part data from '{}' using regex.", scratchcard_data);
    
    let (game_specs, card_vals) = match scratchcard_data.split_once('|') {
        Some(s) => s,
        None => return Ok(0)
    };

    let (_, winning_vals) = match game_specs.split_once(':') {
        Some(s) => s,
        None => return Err("Invalid game data entry, cannot parse.".to_string())
    };

    let number_re = match Regex::new(r"\d+") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern for number read: {}", e))
    };

    let winning_vals_iter: Vec<String> = number_re.find_iter(winning_vals)
        .map(|x| x.as_str().to_string())
        .collect();
    let mut score: i32 = 0;

    for value in number_re.find_iter(card_vals) {
        if winning_vals_iter.iter().find(|&x| x == value.as_str()).is_some() {
            log::debug!("Scoring value {}", value.as_str().to_string());
            let value_int = match value.as_str().parse::<i32>() {
               Ok(v) => v,
               Err(e) => panic!("{}", e)
            };
            println!("{}", score);
            score = scoring(score, value_int);
        }
    }
    Ok(score)
}

fn get_gamecard_scores<F: Fn(i32, i32) -> i32>(card_table_file: &String, scorer: F) -> Result<IndexMap<i32, i32>, String> {
    /* Retrieve the scores for each game in a session of scratch cards.

    For each scratchcard calculates the total score using the provided scoring function.
    
    # Arguments

    * `card_table_file` - file containing lines representing data for each scratchcard.
    * `scoring` - a lambda/function for scoring, the function takes the initial score and the matched value and returns the new total

    # Returns

    total score for each scratchcard as a hashmap

    # Example

    ```
    let scorer = |total, _| return if total < 1 {1} else {total * 2};
        
    get_gamecard_scores((&"/path/to/file".to_string(), &scorer).unwrap();
    ```
    */
    let in_file = match File::open(card_table_file) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", card_table_file, e))
    };
    let file_reader = BufReader::new(in_file);

    let regex_game_id = match Regex::new(r"Card\s+(\d+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern for game ID read: {}", e))
    };

    let mut gamecard_scores = IndexMap::<i32, i32>::new();

    for line in file_reader.lines() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };
        let game_id: i32 = match regex_game_id.captures_iter(&file_line).next() {
            Some(r) => {
                match r.get(1) {
                    Some(g1) => match g1.as_str().parse::<i32>() {
                        Ok(n) => n,
                        Err(e) => return Err(format!("Failed to parse '{}': {}", g1.as_str(), e))
                    },
                    None => continue
                }
            },
            None => continue
        };
        let score = get_scratchcard_score(&file_line, &scorer)?;
        println!("{} {}", game_id, score);    
        gamecard_scores.insert(game_id, score);
    }

    Ok(gamecard_scores)

}

pub fn get_total_gamecards_score<F: Fn(i32, i32) -> i32>(card_table_file: &String, scorer: F) -> Result<i32, String> {
    /* Get the overall total for a session of scratchcards.

    For each scratchcard calculates the total score using the provided scoring function and summates the result.
    
    # Arguments

    * `card_table_file` - file containing lines representing data for each scratchcard.
    * `scoring` - a lambda/function for scoring, the function takes the initial score and the matched value and returns the new total

    # Returns

    total score of all scratchcards

    # Example

    ```
    let scorer = |total, _| return if total < 1 {1} else {total * 2};
        
    get_total_gamecards_score((&"/path/to/file".to_string(), &scorer).unwrap();
    ```
    */
    let gamecard_scores = get_gamecard_scores(&card_table_file, &scorer)?;

    let total_score = gamecard_scores.values().sum();

    Ok(total_score)
}

pub fn get_total_cards_won<F: Fn(i32, i32) -> i32>(card_table_file: &String, scorer: F) -> Result<i32, String> {
    /* For a given set of scratchcard data use the proper scoring system of winning cards per game.

    The alternate scoring system whereby cards are won for each match found, and matches for
    the won cards are also taken into account. The given scoring function is used to find the total score.

     # Arguments

    * `card_table_file` - file containing lines representing data for each scratchcard.
    * `scoring` - a lambda/function for scoring, the function takes the initial score and the matched value and returns the new total

    # Returns

    total score of all cumulative scratchcards after game completion

    # Example

    ```
    let scorer = |total, _| return total + 1;
        
    get_total_cards_won((&"/path/to/file".to_string(), &scorer).unwrap();
    ```
    */
    log::info!("Totaling all cards won this session");

    let gamecard_scores = get_gamecard_scores(&card_table_file, &scorer)?;

    let mut card_counter: HashMap<i32, i32> = gamecard_scores
        .keys()
        .map(|&card_id| (card_id, 1))
        .collect();

    for (card_id, matches) in &gamecard_scores {
        let card_quantity = match card_counter.get(&card_id) {
            Some(sc) => sc.clone(),
            None => return Err(format!("Expected score for card {} but none found", card_id))
        };

        for card_index in card_id + 1..=card_id + matches {
            match card_counter.get_mut(&card_index) {
                Some(v) => {
                    *v += card_quantity;
                },
                None => {
                    card_counter.insert(card_index, card_quantity);
                    ()
                }
            }
        }
    }

    Ok(card_counter.values().sum())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_scoring() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let test_str ="Game N: 34 45 8 81 40 23 | 8 45 9 12 65 23".to_string();

        let scorer = |total, _| return if total < 1 {1} else {total * 2};

        assert_eq!(get_scratchcard_score(&test_str, scorer).unwrap(), 4);
        
    }

    #[test]
    fn test_total_score() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_4.dat");

        let scorer = |total, _| return if total < 1 {1} else {total * 2};
        
        assert_eq!(get_total_gamecards_score(&test_file.to_str().unwrap().to_string(), scorer).unwrap(), 13);
    }

    #[test]
    fn test_total_cards() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_4.dat");

        let scorer = |total, _| return total + 1;
        
        let total_cards = get_total_cards_won(&test_file.to_str().unwrap().to_string(), scorer).unwrap();
        assert_eq!(total_cards, 30);
    }
}