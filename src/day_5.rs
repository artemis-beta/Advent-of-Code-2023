use regex::Regex;
use indexmap::IndexMap;
use std::{fs::read_to_string, io};

fn get_target_seeds(file_entry: &String) -> Result<Vec<i64>, String> {
    let (_, _seed_nums) = match file_entry.split_once(":") {
        Some(s) => s,
        None => return Err(format!("Expected split at ':'"))
    };

    let number_re = match Regex::new(r"\d+") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile number regex: {}", e))
    };

    let mut seeds = Vec::<i64>::new();

    for entry in number_re.find_iter(file_entry) {
        let _value_int = match entry.as_str().parse::<i64>() {
            Ok(v) => seeds.push(v),
            Err(e) => panic!("{}", e)
        };
    }

    Ok(seeds)
}

fn get_conversions(almanac_data: &String) -> Result<IndexMap<String, Vec::<Vec<i32>>>, String> {
    let header_regex = match Regex::new(r"(\w+)-to-(\w+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile header regex: {}", e))
    };

    let number_regex = match Regex::new(r"\d+") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile number regex: {}", e))
    };

    let mut functions = IndexMap::<String, Vec<Vec<i32>>>::new();
    let mut block_str_index_ranges = IndexMap::<(String, String), (i32, i32)>::new();
    let mut lower_limit: i32 = 0;
    let mut key_start: String = String::new();
    let mut key_end: String = String::new();

    for capture in header_regex.captures_iter(almanac_data) {
        let (start, start_index) = match capture.get(1) {
            Some(st) => {
                (st.as_str().to_string(), st.start())
            },
            None => return Err(format!("Expected and category in string"))
        };
        let (end, end_index) = match capture.get(2) {
            Some(st) => {
                (st.as_str().to_string(), st.start())
            },
            None => return Err(format!("Expected and category in string"))
        };

        if lower_limit > 0 {block_str_index_ranges.insert((key_start.clone(), key_end.clone()), (lower_limit, start_index as i32));}

        key_start = start.clone();
        key_end = end.clone();
        lower_limit = end_index as i32;
    }

    block_str_index_ranges.insert((key_start.clone(), key_end.clone()), (lower_limit, almanac_data.len() as i32));

    for ((start, end), (lower_lim, upper_lim)) in block_str_index_ranges {
        let file_lines = almanac_data[lower_lim as usize..upper_lim as usize].lines();

        let mut range_definitions = Vec::<Vec<i32>>::new();

        for line in file_lines {
            let mut range_components = Vec::<i32>::new();
            for number in number_regex.find_iter(line) {
                match number.as_str().parse::<i32>() {
                    Ok(v) => range_components.push(v),
                    Err(e) => panic!("{}", e)
                };
            }
            if range_components.len() > 0 {range_definitions.push(range_components);}
        }

        functions.insert(format!("{}->{}", start, end), range_definitions);
    }
    Ok(functions)
}

fn get_propagated_values(input_range: &(i32, i32), conversions: &IndexMap<String, Vec<Vec<i32>>>) -> Result<Vec<(i32, i32)>, String> {
    log::info!("Propagating range {} <= x < {} ...", input_range.0, input_range.1);

    let conversion_iter = conversions.iter();

    let mut pre_propagated_ranges: Vec<(i32, i32)> = vec![*input_range];
    let mut split_ranges = Vec::<(i32, i32)>::new();
    let mut output_ranges = Vec::<(i32, i32)>::new();

    for (key, ranges) in conversion_iter {
        log::debug!("Running mapping {}", key);
        split_ranges.clear();
        output_ranges.clear();

        for io_range in &pre_propagated_ranges {
            log::debug!("Checking {} <= x < {} for splits", io_range.0, io_range.1);
            for range_def in ranges {
                let source_lower_limit = range_def[1];
                let interval = range_def[2];
                let source_upper_limit = source_lower_limit + interval;

                // If no overlap at all continue
                let max_below_threshold = io_range.1 < source_lower_limit;
                let min_above_threshold = io_range.0 >= source_upper_limit;
                if max_below_threshold || min_above_threshold {continue;}

                if io_range.0 >= source_lower_limit && io_range.1 < source_upper_limit {
                    log::debug!("Preserved ({}, {})", io_range.0, io_range.1);
                    split_ranges.push((io_range.0, io_range.1));
                }

                else if io_range.0 < source_lower_limit {
                    log::debug!("Raised lower limit to: {}", source_lower_limit);
                    split_ranges.push((source_lower_limit, io_range.1));
                }

                else if io_range.1 >= source_upper_limit {
                    log::debug!("Lowered upper limit to: {}", source_upper_limit);
                    split_ranges.push((io_range.0, source_upper_limit));
                }
            }
        }

        if split_ranges.len() > 1 {log::debug!("Split ranges: {:?}", split_ranges);}

        // Now propagate each of the obtained ranges
        for split_range in &split_ranges {
            let mut range_out: (i32, i32) = (0, 0);
            for range_def in ranges {
                let source_lower_limit = range_def[1];
                let dest_lower_limit = range_def[0];
                let interval = range_def[2];

                // If no overlap at all continue
                let max_below_threshold = split_range.1 < source_lower_limit;
                let min_above_threshold = split_range.0 >= source_lower_limit + interval;
                if max_below_threshold || min_above_threshold {continue;}

                if split_range.0 < source_lower_limit || split_range.0 >= source_lower_limit + interval {continue;}

                log::debug!(
                    "Mapping {} <= {} < {} -> {} <= {} < {}",
                    source_lower_limit,
                    split_range.0,
                    source_lower_limit + interval,
                    dest_lower_limit,
                    (dest_lower_limit - source_lower_limit) + split_range.0,
                    dest_lower_limit + interval
                );
                log::debug!(
                    "Mapping {} <= {} < {} -> {} <= {} < {}",
                    source_lower_limit,
                    split_range.1,
                    source_lower_limit + interval,
                    dest_lower_limit,
                    (dest_lower_limit - source_lower_limit) + split_range.1,
                    dest_lower_limit + interval
                );
                range_out.0 = (dest_lower_limit - source_lower_limit) + split_range.0;
                range_out.1 = (dest_lower_limit - source_lower_limit) + split_range.1;
                break;
            }
            output_ranges.push(range_out);
        }

        // If no output ranges, input same as output
        if output_ranges.len() < 1 {
            output_ranges = vec![pre_propagated_ranges[0]];
        }

        log::debug!("Mapping result: {:?}", output_ranges);

        pre_propagated_ranges = output_ranges.clone();
    }

    Ok(output_ranges)
}

pub fn parse_almanac_conversions(file_name: &String, use_ranges: bool) -> Result<Vec<(i32, i32)>, String> {
    let file_str = match read_to_string(file_name) {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e)
    };

    let first_line = match file_str.lines().next() {
        Some(l) => l,
        None => panic!("Failed to obtain number of seeds")
    };

    let intro_parse = get_target_seeds(&first_line.to_string())?;
    let seed_ranges: Vec<(i32, i32)>;

    if use_ranges {
        seed_ranges = intro_parse
            .chunks(2)
            .filter(|x| x.len() == 2)
            .map(|x| x.to_vec())
            .map(|x| (x[0] as i32, (x[0] + x[1]) as i32))
            .collect();
    } else {
        seed_ranges = intro_parse
            .iter()
            .map(|&x| (x as i32, x as i32))
            .collect();
    }
    
    let other_content = file_str[first_line.len()..].to_string();

    let conversions = get_conversions(&other_content)?;

    let mut propagated_values = Vec::<(i32, i32)>::new();


    for range_set in seed_ranges {
        let propagated_value = get_propagated_values(&range_set, &conversions)?;
        propagated_values.extend(propagated_value);
    }

    Ok(propagated_values)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_no_match_returns_same_value() {
        let input= (12, 12);
        let mut conversions = IndexMap::<String, Vec<Vec<i32>>>::new();
        conversions.insert("test".to_string(), vec![vec![23 as i32, 45 as i32, 2 as i32]]);
        let propagated_value = get_propagated_values(&input, &conversions)
            .unwrap()
            .iter()
            .map(|x| x.0)
            .min()
            .unwrap();
        assert_eq!(propagated_value, input.0);
    }

    #[test]
    fn test_single_step() {
        let input = (12, 14);
        let expect = 67;
        let mut conversions = IndexMap::<String, Vec<Vec<i32>>>::new();
        conversions.insert("test".to_string(), vec![vec![65 as i32, 10 as i32, 6 as i32]]);
        let propagated_value = get_propagated_values(&input, &conversions);
        println!("{:?}", propagated_value);
        let temp = propagated_value.unwrap()
            .iter()
            .map(|x| x.0)
            .min()
            .unwrap();
        assert_eq!(temp, expect);
    }

    #[test]
    fn test_minimum_location() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_5.dat");

        let final_value = parse_almanac_conversions(&test_file.to_str().unwrap().to_string(), false).unwrap();

        let minimum_val = final_value.iter().min().unwrap();

        assert_eq!(minimum_val.0, 35);
    }

    #[test]
    fn test_minimum_location_ranges() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_5.dat");

        let final_value = parse_almanac_conversions(&test_file.to_str().unwrap().to_string(), true).unwrap();

        let minimum_val = final_value.iter().min().unwrap();

        assert_eq!(minimum_val.0, 46);
    }
}