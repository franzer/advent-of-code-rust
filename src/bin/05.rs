advent_of_code::solution!(5);

struct Rule {
    before: u32,
    after: u32,
}

fn parse_rule(line: &str) -> Option<Rule> {
    let parts: Vec<&str> = line.split('|').collect();
    if parts.len() == 2 {
        let before = parts[0].parse().ok()?;
        let after = parts[1].parse().ok()?;
        Some(Rule{ before, after })
    }
    else {
        None
    }
}

fn parse_sequence(line: &str) -> Vec<u32> {
    line.split(',')
    .filter_map(|num| num.parse().ok())
    .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total = 0;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Rule> = parts[0].lines().map(|line| parse_rule(line).unwrap()).collect();
    let sequences: Vec<Vec<u32>> = parts[1].lines().map(|line| parse_sequence(line)).collect();

    for item in sequences.iter() {
        let mut is_valid = true;
        for rule in rules.iter() {
            if item.contains(&rule.before) && item.contains(&rule.after) {
                let num1 = item.iter().position(|x| x == &rule.before);
                let num2 = item.iter().position(|x| x == &rule.after);
                if num1 > num2 {
                    is_valid = false;
                    break
                }
            }
        }
        if is_valid {
            let val_to_add = item[item.len() / 2];
            total += val_to_add
        }
    }
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total = 0;
    let parts: Vec<&str> = input.split("\n\n").collect();

    let rules: Vec<Rule> = parts[0].lines().map(|line| parse_rule(line).unwrap()).collect();
    let sequences: Vec<Vec<u32>> = parts[1].lines().map(|line| parse_sequence(line)).collect();


    for item in sequences.iter() {
        let mut final_seq: Vec<u32> = item.clone();
        let mut bad_sec = false;
        loop {
            let mut made_any_swap = false;
            for rule in rules.iter() {
                if final_seq.contains(&rule.before) && final_seq.contains(&rule.after) {
                    let num1 = final_seq.iter().position(|x| x == &rule.before);
                    let num2 = final_seq.iter().position(|x| x == &rule.after);
                    if num1 > num2 {
                        final_seq.swap(num2.unwrap(), num1.unwrap());
                        made_any_swap = true;
                        bad_sec = true;
                    }
                }
            }
            if !made_any_swap {
                break
            }
        }
        if bad_sec{
            let val_to_add = final_seq[final_seq.len() / 2];
            total += val_to_add
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
