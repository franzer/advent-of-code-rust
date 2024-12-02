advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<i32> {
    let mut total: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|number| number.parse().ok()).collect();
        if let Ok(_) = validate_sequence(&nums) {
            total += 1
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total: i32 = 0;

    for line in input.lines() {
        let nums: Vec<i32> = line.split_whitespace().filter_map(|number| number.parse().ok()).collect();
        if let Ok(_) = validate_sequence(&nums) {
            total += 1;
            continue;
        }

       let mut found_valid = false;
       for i in 0..nums.len() {
           let mut test_nums = nums.clone();
           test_nums.remove(i);

           if let Ok(_) = validate_sequence(&test_nums) {
               found_valid = true;
               break;
           }
       }

       if found_valid {
           total += 1;
       }
   }
    Some(total)
}

#[allow(dead_code)]
enum ValidationError {
    InvalidDifference(usize),
    InconsistentDirection(usize),
}

fn validate_sequence(nums: &[i32]) -> Result<bool, ValidationError> {
    let is_increasing = nums[1] > nums[0];

    for (i, window) in nums.windows(2).enumerate() {
        let diff = (window[1] - window[0]).abs();
        if diff < 1 || diff > 3 {
            return Err(ValidationError::InvalidDifference(i + 1));
        }

        if (window[1] > window[0]) != is_increasing {
            return Err(ValidationError::InconsistentDirection(i + 1));
        }
    }

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
