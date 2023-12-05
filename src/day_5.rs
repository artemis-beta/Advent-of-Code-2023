use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn get_target_seeds(file_entry: &String) -> Result<Vec<i32>, String> {
    let (_, seed_nums) = match file_entry.split_once(":") {
        Ok(s) => s,
        Err(e) => return Err(format!("Expected split at ':' but got: {}", e))
    };

    let number_re = match Regex::new(r"\d+") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile number regex: {}", e))
    };

    let mut seeds = Vec::<i32>::new();

    for entry in number_re.find_iter(file_entry) {
        let value_int = match entry.as_str().parse::<i32>() {
            Ok(v) => seeds.push(v),
            Err(e) => panic!("{}", e)
         };
    }

    Ok(seeds)
}

fn get_conversions<F: F(i32)>(almanac_data: &String) -> Result<Vec<F>, String> {
    let header_regex = match Regex::new(r"(\w+)-to-(\w+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile header regex: {}", e))
    };

    for capture in header_regex.captures_iter(almanac_data) {
        
    }
}