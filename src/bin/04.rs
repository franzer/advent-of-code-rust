advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();

    let directions = [
        (0, 1), //right
        (0, -1), //left
        (-1, 0), //up
        (1, 0), //down
        (-1, 1), //up-right
        (-1, -1), //up-left
        (1, 1), //down-right
        (1, -1), //down-left
    ];

    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                if check_word(&grid, row, col, dx, dy) {
                    count += 1;
                }
            }
        }
    }

    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
     let grid: Vec<Vec<char>> = input.lines()
    .map(|line| line.chars().collect())
    .collect();



    let mut count = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == 'A' {
                    if check_x_mas(&grid, row, col) {
                        count += 1
                }
            }
        }
    }

    Some(count)
}

fn check_word(grid: &[Vec<char>], row: usize, col: usize, dx: i32, dy: i32) -> bool {
    let word = "XMAS";
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for (i, expected_char) in word.chars().enumerate() {
        let new_row = row as i32 + dx * i32::try_from(i).unwrap();
        let new_col = col as i32 + dy * i32::try_from(i).unwrap();

        if new_row < 0 || new_row >= rows || new_col < 0 || new_col >= cols {
            return false;
        }

        if grid[new_row as usize][new_col as usize] != expected_char {
            return false;
        }
    }

    true
}

fn check_x_mas(grid: &[Vec<char>], row: usize, col: usize) -> bool {
    let diagonals = [
        [(-1, -1), (1, 1)],
        [(-1, 1), (1, -1)]
    ];

    let mut valid_pair = 0;
    let rows = grid.len() as i32;
    let cols = grid[0].len() as i32;

    for pair in diagonals {
        let (dx1, dy1) = pair[0];
        let (dx2, dy2) = pair[1];

        let row1 = row as i32 + dx1;
        let col1 = col as i32 + dy1;
        let row2 = row as i32 + dx2;
        let col2 = col as i32 + dy2;

        // Check if any position is out of bounds
        if row1 < 0 || row1 >= rows || col1 < 0 || col1 >= cols ||
           row2 < 0 || row2 >= rows || col2 < 0 || col2 >= cols {
            continue;
        }

        let letter1 = grid[row1 as usize][col1 as usize];
        let letter2 = grid[row2 as usize][col2 as usize];

        if (letter1 == 'M' && letter2 == 'S') || (letter1 == 'S' && letter2 == 'M') {
           valid_pair +=1;
        }
    }

    valid_pair == 2

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
