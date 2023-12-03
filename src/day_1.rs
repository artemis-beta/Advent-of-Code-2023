/*                        ADVENT OF CODE DAY 1

Calibration of a snow machine is performed reading a file containing
sets of characters, the first and last numerical values are combined to form
two digit numbers. Advanced calibration also takes into accounts word versions
of numbers, e.g. 'eight'.

The following code uses Regular Expressions to find digits via iterators, and
the find and rfind methods to find word forms.

@author : K. Zarebski
@date : last modified 2023-12-02

*/

use regex::Regex;
use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use log;


fn number_words_in_line(line: &String) -> Option<((usize, i32), (usize, i32))>  {
    /* Returns the first and last word-based numbers within a string if present.

    Uses find and reverse find iterating through the word form of the first
    nine digits. The function returns a pair of pairs each representing the index
    position of the start of the word, and its integer form.

    # Arguments

    * `line` - the string to process for integers

    # Examples

    ```
    let test_string = "3fiveeightoneightg".to_string();
    let first_last_pair = match {
        Some(n) => n,
        None => panic!("Expected number read from words")
    };
    ```
    */
    log::debug!("Finding number words in line '{}'", line);
    let mut convert_dict = HashMap::new();
    convert_dict.insert("zero", 0);
    convert_dict.insert("one", 1);
    convert_dict.insert("two", 2);
    convert_dict.insert("three", 3);
    convert_dict.insert("four", 4);
    convert_dict.insert("five", 5);
    convert_dict.insert("six", 6);
    convert_dict.insert("seven", 7);
    convert_dict.insert("eight", 8);
    convert_dict.insert("nine", 9);

    let mut found_nums: Vec<i32> = Vec::new();
    let mut num_indices: Vec<usize> = Vec::new();

    for (key, value) in convert_dict.iter() {
        match line.find(key) {
            Some(i) => {
                found_nums.push(*value);
                num_indices.push(i)
            },
            None => ()
        };
        match line.rfind(key) {
            Some(i) => {
                found_nums.push(*value);
                num_indices.push(i)
            },
            None => ()
        };
    }

    let min = match num_indices.iter().enumerate().min_by(|(_, &a), (_, &b)| a.cmp(&b)) {
        Some(m) => Some((*m.1, found_nums[m.0])),
        None => None
    };
    let mut max = match num_indices.iter().enumerate().max_by(|(_, &a), (_, &b)| a.cmp(&b)) {
        Some(m) => Some((*m.1, found_nums[m.0])),
        None => None
    };

    if min.is_none() {
        return None;
    }

    if max.is_none() {
        max = min.clone();
    }

    Some((min.unwrap(), max.unwrap()))
}

pub fn calibrate_from_data(calibration_file: &String, allow_str_nums: bool) -> Result<i32, String> {
    /* Perform a calibration using a calibration file.

    A calibration is performed by reading every line of a calibration file. For the basic
    calibration this involves taking the first and last digit of each line, combining these
    into a single two digit value and totaling these.

    For an advanced calibration, the word form of digits is also taken into account,
    i.e. lines containing 'one' etc will be treated as having the relevant digit.

    # Arguments

    * `calibration_file` - path of file for calibration
    * `allow_str_nums` - take into account the word form of digits

    # Examples

    ```
    let total = match calibrate_from_file("/path/to/file.dat", true) {
        Ok(t) => t,
        Err(e) => panic!(e)
    };
    ```
    */
    let re = match Regex::new(r"[0-9]") {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern matching: {}", e))
    };

    let mut total: i32 = 0;
    let in_file = match File::open(calibration_file) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", calibration_file, e))
    };
    let file_reader = BufReader::new(in_file);

    for line in file_reader.lines() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };
        
        let mut digits = re.find_iter(file_line.as_str());

        let mut first_num = match digits.next() {
            Some(n) => n.as_str().to_string(),
            None => "".to_string()
        };

        let mut first_num_index = 1000;
        
        if !first_num.is_empty() {
            first_num_index = match file_line.find(&first_num) {
                Some(i) => i,
                None => return Err(format!("Failed to retrieve index of found number {}", first_num))
            };
        }

        let mut last_num = match digits.last() {
            Some(n) => n.as_str().to_string(),
            None => first_num.clone()
        };

        let last_num_index = match file_line.rfind(&last_num) {
            Some(i) => {if last_num.is_empty() {0} else {i}},
            None => return Err(format!("Failed to retrieve index of found number {}", last_num))
        };

        if allow_str_nums {
            match number_words_in_line(&file_line) {
                Some(n) => {
                    first_num = if first_num_index < n.0.0 {first_num.to_string()} else {n.0.1.to_string()};
                    last_num = if last_num_index > n.1.0 {last_num.to_string()} else {n.1.1.to_string()};
                },
                None => ()
            };
        }

        let num_str = format!("{}{}", first_num, if last_num.is_empty() {first_num.clone()} else {last_num.clone()});

        match num_str.parse::<i32>() {
            Ok(n) => {
                log::info!("Found number: {}", n);
                total += n;
            },
            Err(e) => return Err(format!("Failed to parse '{}': {}", num_str, e))
        };
    }
    Ok(total)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_calibration_no_words() -> () {
        match simple_logger::init_with_env() {
        Ok(l) => l,
        Err(_) => ()
        };
        
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_1_1.dat");
        assert_eq!(calibrate_from_data(&test_file.to_str().unwrap().to_string(), false).unwrap(), 142);
    }

    #[test]
    fn test_overlapped_words_and_repeat() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
            
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/test_case_1.dat");
        assert_eq!(calibrate_from_data(&test_file.to_str().unwrap().to_string(), true).unwrap(), 38);
    }

    #[test]
    fn test_calibration_words() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
        
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_1_2.dat");
        assert_eq!(calibrate_from_data(&test_file.to_str().unwrap().to_string(), true).unwrap(), 281);
    }
}