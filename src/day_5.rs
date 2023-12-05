use regex::Regex;
use indexmap::IndexMap;
use std::fs::read_to_string;

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

fn get_conversions(almanac_data: &String) -> Result<IndexMap<String, Vec::<Vec<usize>>>, String> {
    let header_regex = match Regex::new(r"(\w+)-to-(\w+)") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile header regex: {}", e))
    };

    let number_regex = match Regex::new(r"\d+") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to compile number regex: {}", e))
    };

    let mut functions = IndexMap::<String, Vec<Vec<usize>>>::new();
    let mut block_ranges = IndexMap::<(String, String), (usize, usize)>::new();
    let mut lower_limit: usize = 0;
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

        if lower_limit > 0 {block_ranges.insert((key_start.clone(), key_end.clone()), (lower_limit, start_index));}

        key_start = start.clone();
        key_end = end.clone();
        lower_limit = end_index;
    }

    block_ranges.insert((key_start.clone(), key_end.clone()), (lower_limit, almanac_data.len()));

    for ((start, end), (lower_lim, upper_lim)) in block_ranges {
        let file_lines = almanac_data[lower_lim..upper_lim].lines();

        let mut range_definitions = Vec::<Vec<usize>>::new();

        for line in file_lines {
            let mut range_components = Vec::<usize>::new();
            for number in number_regex.find_iter(line) {
                match number.as_str().parse::<usize>() {
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

fn get_propagated_value(input: &i64, conversions: &IndexMap<String, Vec<Vec<usize>>>) -> Result<i64, String> {
    log::debug!("Propagating {}...", input);
    let mut final_val = *input;

    let conversion_iter = conversions.iter();

    for (_, ranges) in conversion_iter {
        for range_def in ranges {
            let source_lower_limit = range_def[1] as i64;
            let dest_lower_limit = range_def[0] as i64;
            let interval = range_def[2] as i64;
            if final_val < source_lower_limit || final_val >= source_lower_limit + interval {continue;}
            log::debug!(
                "Comparing {} <= {} < {} -> {} <= {} < {}",
                source_lower_limit,
                final_val,
                source_lower_limit + interval,
                dest_lower_limit,
                (dest_lower_limit - source_lower_limit) + final_val,
                dest_lower_limit + interval
            );
            final_val += dest_lower_limit - source_lower_limit;
            break;
        }
    }
    Ok(final_val as i64)
}

pub fn parse_almanac_conversions(file_name: &String, use_ranges: bool) -> Result<Vec<i64>, String> {
    let file_str = match read_to_string(file_name) {
        Ok(contents) => contents,
        Err(e) => panic!("{}", e)
    };

    let first_line = match file_str.lines().next() {
        Some(l) => l,
        None => panic!("Failed to obtain number of seeds")
    };

    let intro_parse = get_target_seeds(&first_line.to_string())?;
    let mut seeds = intro_parse.clone();

    if use_ranges {
        seeds = intro_parse
            .chunks(2)
            .filter(|x| x.len() == 2)
            .map(|x| x.to_vec())
            .map(|x| (x[0]..x[0] + x[1]))
            .flatten()
            .collect();
    }
    
    let other_content = file_str[first_line.len()..].to_string();

    let conversions = get_conversions(&other_content)?;

    let propagated_values: Vec<i64> = seeds.iter()
        .map(|x| get_propagated_value(x, &conversions).unwrap())
        .collect();

    if use_ranges {
        Ok(
            propagated_values
            .chunks(2)
            .filter(|x| x.len() == 2)
            .map(|x| x.to_vec())
            .flatten()
            .collect()
        )
    } else {
        Ok(propagated_values)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_no_match_returns_same_value() {
        let input: i64 = 12;
        let mut conversions = IndexMap::<String, Vec<Vec<usize>>>::new();
        conversions.insert("test".to_string(), vec![vec![23 as usize, 45 as usize, 2 as usize]]);
        assert_eq!(get_propagated_value(&input, &conversions).unwrap(), input);
    }

    #[test]
    fn test_single_step() {
        let input: i64 = 12;
        let expect: i64 = 67;
        let mut conversions = IndexMap::<String, Vec<Vec<usize>>>::new();
        conversions.insert("test".to_string(), vec![vec![65 as usize, 10 as usize, 6 as usize]]);
        assert_eq!(get_propagated_value(&input, &conversions).unwrap(), expect);
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

        let minimum_val = final_value.iter().min();

        assert_eq!(*minimum_val.unwrap(), 35);
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

        let minimum_val = final_value.iter().min();

        assert_eq!(*minimum_val.unwrap(), 46);
    }
}