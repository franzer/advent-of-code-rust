advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sums = 0;
    let mut list1:Vec<i32> = vec![];
    let mut list2 = vec![];

    for line in input.lines() {
        list1.push(line[0..5].to_string().parse().unwrap());
        list2.push(line[8..13].to_string().parse().unwrap());
    }
    list1.sort();
    list2.sort();

    for (item1, item2) in list1.iter().zip(list2.iter()) {
        if item1 > item2 {
            sums += item1 - item2
        } else {
            sums += item2 - item1
        }
    }

    Some(sums.try_into().unwrap())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1:Vec<i32> = vec![];
    let mut list2:Vec<i32> = vec![];

    for line in input.lines() {
        list1.push(line[0..5].to_string().parse().unwrap());
        list2.push(line[8..13].to_string().parse().unwrap());
    }

    let mut total_matches = 0;
    for item1 in list1.iter() {
        let count = list2.iter().filter(|&item2| item2 == item1).count();
        total_matches += (count as i32) * item1 ;
    }

    Some(total_matches.try_into().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
