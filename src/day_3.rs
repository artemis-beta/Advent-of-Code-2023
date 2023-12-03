/*                        ADVENT OF CODE DAY 3

The blueprint for a gondola system is presented as inventory numbers arranged
in rows and offset in position. If the number is neighboured by a symbol not 
including '.' it is a part number. Furthermore if this symbol is '*' and the
symbol has exactly two neighbouring numbers in total, then the part is a gear.

The gear ratio is defined as the product of the two numbers either side of the
'*' symbol.

@author : K. Zarebski
@date : last modified 2023-12-03

*/

use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use log;

fn get_objects(regex_str: &str, blueprint_file: &String) -> Result<(Vec<String>, Vec<(usize, usize)>), String> {
    /* Retrieve objects from a file matching the given regular expression.

    The retrieved objects include the symbols found and the coordinates of their locations.

    # Arguments

    * `regex_str` - a regular expression defining the objects to search for.
    * `blueprint_file` - the input blueprint data file to search.

    # Returns

    A pair containing two vectors of equal length:
        - The objects found
        - The coordinates of the object start positions

    # Example

    ```
    let (symbol_strs, symbol_coords) = get_objects(r"[^\d\.]", &blueprint_file)?;
    ```
    */
    log::debug!("Reading part data from '{}' using regex '{}'", blueprint_file, regex_str);
    let re = match Regex::new(regex_str) {
        Ok(r) => r,
        Err(e) => return Err(format!("Failed to initialise regex pattern: {}", e))
    };

    let in_file = match File::open(blueprint_file) {
        Ok(o) => o,
        Err(e) => return Err(format!("Failed to open file '{}': {}", blueprint_file, e))
    };

    let file_reader = BufReader::new(in_file);

    let mut coords: Vec<(usize, usize)> = Vec::<(usize, usize)>::new();
    let mut obj_strs = Vec::<String>::new();
    for (i, line) in file_reader.lines().enumerate() {
        let file_line = match line {
            Ok(f) => f,
            Err(e) => return Err(format!("Bad file line: {}", e))
        };

        for number in re.find_iter(file_line.as_str()) {
            coords.push((i, number.start()));
            obj_strs.push(number.as_str().to_string());
        }
    }
    Ok((obj_strs, coords))
}

fn get_object_neighbour_coords(row: usize, column: usize, length: usize) -> Vec<(usize, usize)> {
    /* Retrieve all possible neighbour coordinates for an object of a given length at a specified coordinate.

    This function looks for all possible coordinates not including negatives that surround an object
    orientated in the horizontal direction:
    
    ...xxxxxx....
    ..xOBJECTx...
    ...xxxxxx....

    OBJECTx......
    xxxxxx.......
    .............

    xxxxxx.......
    OBJECTx......
    xxxxxx.......

    # Arguments

    * `row` - the row coordinate of the object
    * `column` - the column coordinate of the object
    * `length` - the length of the object in the horizontal direction

    # Returns

    A vector containing all coordinates of neighbouring positions as (i32, i32) pairs.

    # Example

    ```
   get_object_neighbour_coords(0, 0, 3);
    ```

    */
    let mut neighbour_values = Vec::<(usize, usize)>::new();
    let mut lower_col_bound = column;

    // If the column number is greater than zero we can include
    // the previous column in neighbours
    if column > 0 {
        neighbour_values.push((row, column - 1));
        lower_col_bound -= 1;
    }

    neighbour_values.push((row, column + length));

    // Add all positions above and below the object
    for col in lower_col_bound..=column + length {
        if row > 0 {
            neighbour_values.push((row - 1, col));
        }

        neighbour_values.push((row + 1, col));
    }

    neighbour_values
}

pub fn get_part_numbers(blueprint_file: &String) -> Result<Vec<i32>, String> {
    /* Get all numbers within a blueprint file that are part numbers.

    Returns all numbers which have at least one neighbouring symbol, as as such
    are defined as part numbers.

    # Arguments

    * `blueprint_file` - file containing blueprint data


    # Returns

    A vector containing all number identifiers for parts.

    # Example

    ```
    let part_numbers = get_part_numbers(&"/path/to/file".to_string()).unwrap();
    ```
    
    */
    log::debug!("Finding number and symbol positions");

    let (_, symbol_coords) = get_objects(r"[^\d\.]", &blueprint_file)?;
    let (number_strs, number_coords) = get_objects(r"\d+", &blueprint_file)?;

    log::debug!("Determining numerical values for numbers identified as part numbers");
    let mut part_numbers = Vec::<i32>::new();

    for (num_str, coord) in number_strs.iter().zip(&number_coords) {

        // Firstly check if the number has a neighbouring symbol in the same row
        if (coord.1 > 0 && symbol_coords.contains(&(coord.0, coord.1-1))) || symbol_coords.contains(&(coord.0, coord.1 + num_str.len())) {
            let integer_num = match num_str.parse::<i32>() {
                Ok(n) => n,
                Err(e) => return Err(format!("Failed to parse number '{}': {}", num_str, e))
            };
            part_numbers.push(integer_num);
            continue;
        }

        let lower_limit = if coord.1 > 0 {coord.1 - 1} else {coord.1};


        // Next check if it has a neighbouring symbol in the row above and below
        for col_num in lower_limit..=coord.1 + num_str.len() {
            if (coord.0 > 0 && symbol_coords.contains(&(coord.0 - 1, col_num))) || symbol_coords.contains(&(coord.0 + 1, col_num)) {
                let integer_num = match num_str.parse::<i32>() {
                    Ok(n) => n,
                    Err(e) => return Err(format!("Failed to parse number '{}': {}", num_str, e))
                };
                part_numbers.push(integer_num);
                break;
            }
        }
    }

    Ok(part_numbers)
}


fn get_gear_neighbours(blueprint_file: &String, gear_symbol: &String) -> Result<Vec<Vec<i32>>, String> {
    /* Get the neighbouring number objects to a all gear objects defined within a blueprint file.

    For a given blueprint file extract all gear symbol positions, then return for each the pair of numbers
    associated with that gear. Gears are defined as having only two neighbouring numbers.

    # Arguments

    * `blueprint_file` - file containing blueprint data.
    * `gear_symbol` - the symbol representing a single gear.

    # Returns

    A vector containing for each gear the two numbers position either side of it.

    # Example
    
    ```
    let gear_neighbours = get_gear_neighbours(&"/path/to/file".to_string(), &"*".to_string())?;
    ```
    */
    log::debug!("Finding number and symbol positions");

    let (symbol_strs, symbol_coords) = get_objects(r"[^\d\.]", &blueprint_file)?;
    let (number_strs, number_coords) = get_objects(r"\d+", &blueprint_file)?;


    let gear_coords: Vec<(usize, usize)> = symbol_coords
        .iter()
        .enumerate()
        .filter(|(i, _)| &symbol_strs[*i] == gear_symbol)
        .map(|(_, &a)| a)
        .collect();

    let mut gear_neighbours = vec![Vec::<i32>::new(); gear_coords.len()];

    for (i, gear_coord) in gear_coords.iter().enumerate() {
        for (number, number_coord) in number_strs.iter().zip(&number_coords) {
            if get_object_neighbour_coords(number_coord.0, number_coord.1, number.len()).contains(&gear_coord) {
                let integer_num = match number.parse::<i32>() {
                    Ok(n) => n,
                    Err(e) => return Err(format!("Failed to parse number '{}': {}", number, e))
                };
                gear_neighbours[i].push(integer_num);
            }
        }
    }
    Ok(gear_neighbours)
}


pub fn get_gear_ratios(blueprint_file: &String, gear_symbol: &String) -> Result<Vec<i32>, String> {
    /* Geat the gear ratios for each gear within a blueprint file.

    For a given blueprint file return the gear ratio for each gear defined within it, this ratio
    is defined as the product of the two object numbers positioned either side of it.

    # Arguments

    * `blueprint_file` - the file containing the blueprint data.
    * `gear_symbol` - the symbol representing a single gear.

    # Returns

    A vector containing the gear ratio for each gear within the blueprint file.


    # Example

    ```
    let gear_neighbours = get_gear_ratios(&"/path/to/file".to_string(), &"*".to_string()).unwrap();
    ```
    */
    let gear_neighbours = get_gear_neighbours(blueprint_file, gear_symbol)?;

    let gear_ratios: Vec<i32> = gear_neighbours
        .iter()
        .filter(|&x| x.len() == 2)
        .map(|x| x.iter().product())
        .collect();

    Ok(gear_ratios)
}


#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_get_number_neighbour_coords() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
        let expected = vec![(0, 3), (1, 0), (1, 1), (1, 2), (1, 3)];

        let neighbours = get_object_neighbour_coords(0, 0, 3);

        for coord in expected {
            log::debug!("Check coord {:?} in {:?}", coord, neighbours);
            assert!(neighbours.contains(&coord));
        }
    }

    #[test]
    fn test_get_part_numbers() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
        let expected = vec![467, 35, 633, 617, 592, 755, 664, 598];
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_3.dat");
        let part_numbers = get_part_numbers(&test_file.to_str().unwrap().to_string()).unwrap();

        for number in expected {
            log::info!("Checking number {}", number);
            assert!(part_numbers.contains(&number));
        }
    }

    #[test]
    fn test_get_gear_ratios() {
        match simple_logger::init_with_env() {
            Ok(l) => l,
            Err(_) => ()
        };
        let mut test_file = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_file.push("data/test/day_3.dat");
        let gear_neighbours = get_gear_ratios(&test_file.to_str().unwrap().to_string(), &"*".to_string()).unwrap();

        let total: i32 = gear_neighbours.iter().sum::<i32>();

        assert_eq!(total, 467835);
    }
}