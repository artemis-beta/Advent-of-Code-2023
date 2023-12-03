use std::path::PathBuf;
use std::collections::HashMap;
use log;
use simple_logger;

fn main() -> () {
    simple_logger::init_with_env().unwrap();
        
    let mut data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_file.push("data/day_2.dat");

    let file_name = match data_file.to_str() {
        Some(f) => f.to_string(),
        None => panic!("{}", "Failed to create file path")
    };

    let mut cubes = HashMap::new();
    cubes.insert(aoc23::day_2::Color::Red, 12);
    cubes.insert(aoc23::day_2::Color::Green, 13);
    cubes.insert(aoc23::day_2::Color::Blue, 14);

    let valid_games_id_total = match aoc23::day_2::get_total_of_permitted_game_ids(&file_name, &cubes) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };

    let total_game_power = match aoc23::day_2::get_total_game_power(&file_name) {
        Ok(t) => t,
        Err(e) => panic!("{}", e)
    };

    log::info!("Using game session data from file {} the total of all valid game IDs is {}", file_name, valid_games_id_total);
    log::info!("For all games, the total game power is {}", total_game_power);
}