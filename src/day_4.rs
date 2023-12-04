use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_scratchcard_score<F: Fn(i32, i32) -> i32>(scratchcard_data: &String, scoring: F) -> Result<i32, String> {
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
            score = scoring(score, value_int);
        }
    }
    Ok(score)
}

pub fn get_total_gamecards_score<F: Fn(i32, i32) -> i32>(card_table_file: &String, scorer: F) -> Result<i32, String> {
    let in_file = match File::open(card_table_file) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", card_table_file, e))
    };
    let file_reader = BufReader::new(in_file);

    let mut total_score: i32 = 0;

    for line in file_reader.lines() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };
        total_score += get_scratchcard_score(&file_line, &scorer)?;
    }

    Ok(total_score)

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
}