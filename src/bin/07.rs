advent_of_code::solution!(7);
use itertools::Itertools;
use rayon::prelude::*;


fn eval_sequence(target: u64, nums:&[u64], operators:&[char]) -> bool {

    itertools::repeat_n(operators, nums.len() -1)
    .multi_cartesian_product()
    .any(|ops| {
        let mut result = nums[0];

        for (op, &num) in ops.iter().zip(&nums[1..]) {
            match op {
                '+' => result += num,
                '*' => result *= num,
                '|' => {
                    let temp_result = result.to_string() + &num.to_string();
                    result = temp_result.parse::<u64>().ok().unwrap_or(u64::MAX);
                }
                _ => unreachable!()
            }
        }

        result == target
    })

}


pub fn part_one(input: &str) -> Option<u64> {
    let equations = input.lines().filter_map(|line| {
        let (target, nums) = line.split_once(':')?;
        let target = target.trim().parse::<u64>().ok()?;
        let nums: Vec<u64> = nums
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        Some((target, nums))
    }).collect::<Vec<_>>();

    let sum: u64 = equations.into_par_iter()
    .filter(|(target, nums)| {
        eval_sequence(*target, nums, &['*', '+'])
    }).map(|(target, _)| target).sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let equations = input.lines().filter_map(|line| {
        let (target, nums) = line.split_once(':')?;
        let target = target.trim().parse::<u64>().ok()?;
        let nums: Vec<u64> = nums
            .split_whitespace()
            .filter_map(|n| n.parse().ok())
            .collect();

        Some((target, nums))
    }).collect::<Vec<_>>();

    let sum: u64 = equations.into_par_iter()
    .filter(|(target, nums)| {
        eval_sequence(*target, nums, &['*', '+'])||
        eval_sequence(*target, nums, &['*', '+', '|'])
    }).map(|(target, _)| target).sum();

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
