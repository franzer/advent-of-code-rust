advent_of_code::solution!(3);

use std::{collections::btree_map::Values, result};

use regex::Regex;
use once_cell::sync::Lazy;

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    static REGEX_CORRECTIONS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap());


    let vector_match: Vec<Vec<&str>> = REGEX_CORRECTIONS
    .captures_iter(input)
    .map(|c| c.iter().map(|m| m.unwrap().as_str()).collect())
    .collect();

    for item in vector_match.iter() {
        if item.len() >= 3 {
            let num1: u32 = item[1].parse().unwrap();
            let num2: u32 = item[2].parse().unwrap();
            total += num1 * num2
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total = 0;
    let mut toggle = true;
    static REGEX_CORRECTIONS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap());

    let vector_match: Vec<&str> = REGEX_CORRECTIONS
        .find_iter(input)
        .map(|m| m.as_str())
        .collect();

    for item in vector_match.iter() {
        match process_item(item) {
            Ok(value) => {
                if toggle {
                    total += value;
                }
            }
            Err(true) => {
                toggle = true;
            }
            Err(false) => {
                toggle = false;
            }
        }
    }
    Some(total)
}


fn process_item(item: &&str) -> Result<i32, bool> {
    match *item {
        s if s.starts_with("mul(") => {
            let nums: Vec<&str> = s[4..s.len()-1].split(',').collect();
            let x = nums[0].parse::<i32>().unwrap();
            let y = nums[1].parse::<i32>().unwrap();
            Ok(x * y)
        }
        "do()" => Err(true),
        "don't()" => Err(false),
        _ => panic!("Something went wrong?: {:}", item)
    }


}

#[cfg(test)]
mod tests {
    use super::*;

//    #[test]
//    fn test_part_one() {
//        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
//        assert_eq!(result, Some(161));
//    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
