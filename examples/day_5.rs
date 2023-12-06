use std::path::PathBuf;
use log;
use simple_logger;

fn main() -> () {
    let skip_part_2 = true;

    simple_logger::init_with_env().unwrap();
        
    let mut data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_file.push("data/day_5.dat");

    let file_name = match data_file.to_str() {
        Some(f) => f.to_string(),
        None => panic!("{}", "Failed to create file path")
    };

    let final_value_no_range = aoc23::day_5::parse_almanac_conversions(&file_name, false).unwrap();


    let minimum_val_no_range = match final_value_no_range.iter().min() {
        Some(m) => m,
        None => panic!("Failed to retrieve minimum value")
    };

    log::info!("For the almanac data given in '{}' the minimum seed location is {}", file_name, minimum_val_no_range.0);

    if skip_part_2 {
        log::warn!("Skipping part 2 as inefficient..");
        return;
    }

    let final_value_range = aoc23::day_5::parse_almanac_conversions(&file_name, true).unwrap();

    let minimum_val_range = match final_value_range.iter().min() {
        Some(m) => m,
        None => panic!("Failed to retrieve minimum value")
    };

    log::info!("If the seed values actually specify ranges, the minimum seed location is {}", minimum_val_range.0);
}