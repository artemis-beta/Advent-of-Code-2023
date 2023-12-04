use std::path::PathBuf;
use std::collections::HashMap;
use log;
use simple_logger;

fn main() -> () {
    simple_logger::init_with_env().unwrap();
        
    let mut data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_file.push("data/day_4.dat");

    let file_name = match data_file.to_str() {
        Some(f) => f.to_string(),
        None => panic!("{}", "Failed to create file path")
    };

    let scorer = |total, _| return if total < 1 {1} else {total * 2};

    let total_score = match aoc23::day_4::get_total_gamecards_score(&file_name, scorer) {
        Ok(t) => t,
        Err(e) => panic!("{}", e)
    };

    log::info!("For the set of game cards given in '{}', the total score using doubling is {}", file_name, total_score);
}