advent_of_code::solution!(8);
use std::collections::{HashMap, HashSet};



fn is_collinear(p1: (i32, i32), p2: (i32, i32), p3: (i32, i32)) -> bool {
    (p2.1 - p1.1) * (p3.0 - p1.0) == (p3.1 - p1.1) * (p2.0 - p1.0)
}

fn manhattan_distance(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

fn find_antinodes(positions: &[(usize, usize)], bounds: (usize, usize), check_distance: bool) -> Vec<(usize, usize)> {
    let mut antinodes = Vec::new();
    let positions: Vec<(i32, i32)> = positions.iter()
        .map(|&(y, x)| (y as i32, x as i32))
        .collect();

    println!("\nChecking positions: {:?}", positions);

    if !check_distance && positions.len() > 1 {
        println!("Adding antenna positions as antinodes");
        antinodes.extend(positions.iter().map(|&(y, x)| (y as usize, x as usize)));
    }

    'outer: for y in 0..bounds.0 as i32 {
        'point: for x in 0..bounds.1 as i32 {
            let p = (y, x);

            if !check_distance {
                let mut collinear_count = 0;
                let mut collinear_with = Vec::new();


                for i in 0..positions.len() {
                    for j in (i+1)..positions.len() {
                        if is_collinear(positions[i], positions[j], p) {
                            collinear_with.push((positions[i], positions[j]));
                            collinear_count += 1;

                            // If we found a collinear pair, this is an antinode
                            if collinear_count > 0 {
                                println!("Found point {:?} collinear with {:?}", p, collinear_with);
                                antinodes.push((y as usize, x as usize));
                                continue 'point;
                            }
                        }
                    }
                }
            }
        }
    }

    println!("Found antinodes: {:?}", antinodes);
    antinodes
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != '.' {
                antennas.entry(*col)
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    let bounds = (grid.len(), grid[0].len());
    let mut antinodes = HashSet::new();

    for (_, positions) in antennas.iter() {
        if positions.len() >= 2 {
            antinodes.extend(find_antinodes(positions, bounds, true));
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut antennas = HashMap::new();
    for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col != '.' {
                antennas.entry(*col)
                    .or_insert_with(Vec::new)
                    .push((i, j));
            }
        }
    }

    let bounds = (grid.len(), grid[0].len());
    let mut antinodes = HashSet::new();

    for (_, positions) in antennas.iter() {
        if positions.len() >= 2 {
            antinodes.extend(find_antinodes(positions, bounds, false));
        }
    }

    Some(antinodes.len() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
