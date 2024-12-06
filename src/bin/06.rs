advent_of_code::solution!(6);
use crate::Direction::Up;
use std::collections::HashSet;

struct Guard {
    position: (i32, i32),
    direction: Direction,
}

impl Guard {
    fn next_position(&self) -> (i32, i32) {
        let delta = self.direction.delta();
        (self.position.0 + delta.0, self.position.1 + delta.1)
    }

    fn turn_right(&mut self) {
        self.direction = self.direction.turn_right();
    }
}

#[derive(Copy, Clone, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut visited = HashSet::new();

    let mut starting_pos = (0, 0);
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                starting_pos = (i as i32, j as i32);
                break 'outer
            }
        }
    }

    let mut guard: Guard = Guard { position: starting_pos, direction: Up };
    visited.insert(guard.position);

    loop {
        let next_pos:(i32, i32) = guard.next_position();

        if next_pos.0 < 0 || next_pos.0 >= grid.len().try_into().unwrap() || next_pos.1 < 0 || next_pos.1 >= grid[0].len().try_into().unwrap() {
            break
        }

        if grid[next_pos.0 as usize][next_pos.1 as usize] != '#' {
            guard.position = next_pos;
            visited.insert(guard.position);
        } else {
            guard.turn_right();
        }
    }


    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<i32> {
    let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

    let mut starting_pos = (0, 0);
    'outer: for (i, row) in grid.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == '^' {
                starting_pos = (i as i32, j as i32);
                break 'outer
            }
        }
    }

    let mut got_stuck_count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if (i as i32, j as i32) == starting_pos {
                continue
            }
            let mut visited:HashSet<((i32, i32), Direction)> = HashSet::new();
            let mut test_grid = grid.clone();
            test_grid[i][j] = '#';
            let mut guard: Guard = Guard { position: starting_pos, direction: Up };
            visited.insert((guard.position, guard.direction));

            loop {
                let next_pos:(i32, i32) = guard.next_position();

                if next_pos.0 < 0 || next_pos.0 >= grid.len().try_into().unwrap() || next_pos.1 < 0 || next_pos.1 >= grid[0].len().try_into().unwrap() {
                    break
                }

                if test_grid[next_pos.0 as usize][next_pos.1 as usize] != '#' {
                    guard.position = next_pos;
                    if visited.contains(&(guard.position, guard.direction)) {
                        got_stuck_count += 1;
                        break;
                    }
                    visited.insert((guard.position, guard.direction));
                } else {
                    guard.turn_right();
                    if visited.contains(&(guard.position, guard.direction)) {
                        got_stuck_count += 1;
                        break;
                    }
                    visited.insert((guard.position, guard.direction));
                }
            }

        }
    }
    Some(got_stuck_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
