advent_of_code::solution!(11);
use std::collections::HashMap;

fn count_results(num: u64, blinks: u32, cache: &mut HashMap<(u64, u32), u64>) -> u64 {
    // Check cache first
    if let Some(&result) = cache.get(&(num, blinks)) {
        //println!("Cache hit: ({}, {}) -> {}", num, blinks, result);
        return result;
    }

    // Base case
    if blinks == 0 {
        //println!("Base case: {} with 0 blinks -> 1", num);
        return 1;
    }

    let result = if num == 0 {
        //println!("Processing 0 -> 1 with {} blinks remaining", blinks - 1);
        count_results(1, blinks - 1, cache)
    } else {
        let num_str = num.to_string();
        if num_str.len() % 2 == 0 {
            let mid = num_str.len() / 2;
            let first_half: u64 = num_str[..mid].parse().unwrap();
            let second_half: u64 = num_str[mid..].parse().unwrap();
            //println!("Splitting {} into {} and {} with {} blinks remaining",
                //num, first_half, second_half, blinks - 1);
            let first_result = count_results(first_half, blinks - 1, cache);
            let second_result = count_results(second_half, blinks - 1, cache);
            //println!("Combined results: {} + {} = {}",
                //first_result, second_result, first_result + second_result);
            first_result + second_result
        } else {
            //println!("Multiplying {} by 2024 = {} with {} blinks remaining",
                //num, num * 2024, blinks - 1);
            count_results(num * 2024, blinks - 1, cache)
        }
    };

    //println!("Caching: ({}, {}) -> {}", num, blinks, result);
    cache.insert((num, blinks), result);
    result
}

pub fn part_one(input: &str) -> Option<u64> {
    let numbers: Vec<u64> = input.split(' ')
        .map(|num| num.parse().unwrap()).collect();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let total: u64 = numbers.iter()
        .map(|&num| count_results(num, 25, &mut cache))
        .sum();

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let numbers: Vec<u64> = input.split(' ')
        .map(|num| num.parse().unwrap()).collect();

    let mut cache: HashMap<(u64, u32), u64> = HashMap::new();
    let total: u64 = numbers.iter()
        .map(|&num| count_results(num, 75, &mut cache))
        .sum();

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

//    #[test]
//    fn test_part_two() {
//        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
//        assert_eq!(result, None);
//    }
}
