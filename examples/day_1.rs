use std::path::PathBuf;
use log;
use simple_logger;

fn main() -> () {
    simple_logger::init_with_env().unwrap();
        
    let mut data_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    data_file.push("data/day_1.dat");

    let file_name = match data_file.to_str() {
        Some(f) => f.to_string(),
        None => panic!("{}", "Failed to create file path")
    };

    let calibration_result = match aoc23::day_1::calibrate_from_data(&file_name, false) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };
    let calibration_result_w_words = match aoc23::day_1::calibrate_from_data(&file_name, true) {
        Ok(c) => c,
        Err(e) => panic!("{}", e)
    };

    log::info!("Using calibration data from file {} the total calibration value is {}", file_name, calibration_result);
    log::info!("Taking into account numbers as words, the new total is {}", calibration_result_w_words);
}