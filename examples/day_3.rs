use std::path::PathBuf;
use log;
use simple_logger;

fn main() -> () {
    simple_logger::init_with_env().unwrap();
    
    let gear_symbol = "*".to_string();
    let mut data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_file.push("data/day_3.dat");

    let file_name = match data_file.to_str() {
        Some(f) => f.to_string(),
        None => panic!("{}", "Failed to create file path")
    };

    let part_numbers = match aoc23::day_3::get_part_numbers(&file_name) {
        Ok(n) => n,
        Err(e) => panic!("{}", e)
    };

    let gear_ratios = match aoc23::day_3::get_gear_ratios(&file_name, &gear_symbol) {
        Ok(n) => n,
        Err(e) => panic!("{}", e)
    };

    log::info!("The total of all part numbers given in the file '{}' is {}", file_name, part_numbers.iter().sum::<i32>());
    log::info!("For all gears represented by the symbol '{}' and having two neighbouring parts, the total of all gear ratios is {}", gear_symbol, gear_ratios.iter().sum::<i32>());
}