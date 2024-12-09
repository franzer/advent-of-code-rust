advent_of_code::solution!(9);

struct File {
    id: usize,
    size: u32,
    spaces_after: u32,
}

fn setup_disk(input: &str) -> (Vec<File>, Vec<Option<usize>>) {
    let input = input.trim();
    let numbers: Vec<u32> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    let mut files: Vec<File> = Vec::new();

    // Create first file
    let first_file: File = File {
        id: 0,
        size: numbers[0],
        spaces_after: numbers[1]
    };
    files.push(first_file);

    // Create remaining files
    for (id, chunk) in numbers[2..].chunks(2).enumerate() {
        let file: File = File {
            id: id + 1,
            size: chunk[0],
            spaces_after: if chunk.len() > 1 { chunk[1] } else { 0 },
        };
        files.push(file);
    }

    // Calculate total disk size
    let total_size: usize = files.iter()
        .map(|f| (f.size + f.spaces_after) as usize)
        .sum();

    // Create and initialize disk
    let mut disk: Vec<Option<usize>> = vec![None; total_size];
    let mut current_position = 0;
    for file in &files {
        for _ in 0..file.size {
            disk[current_position] = Some(file.id);
            current_position += 1;
        }
        current_position += file.spaces_after as usize;
    }

    (files, disk)
}

pub fn part_one(input: &str) -> Option<u64> {
   let mut sum: u64 = 0;
   let (_files, mut disk) = setup_disk(input);

    while disk.contains(&None) {
        let rightmost_pos = disk.iter()
        .rposition(|pos| pos.is_some())
        .unwrap();

        let target_pos = disk.iter()
        .position(|pos| pos.is_none())
        .unwrap();

        if target_pos > rightmost_pos {
            break;
        }

        let file_id = disk[rightmost_pos].unwrap();
        disk[target_pos] = Some(file_id);
        disk[rightmost_pos] = None;
    }

    for (i, num) in disk.iter().enumerate() {
        if let Some(file_id) = num {
            sum += i as u64 * *file_id as u64;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    let (files, mut disk) = setup_disk(input);


    // Process files in reverse order
    for file_id in (0..files.len()).rev() {
        let file_size = files[file_id].size as usize;

        let file_positions: Vec<usize> = disk.iter()
            .enumerate()
            .filter(|(_, &pos)| pos == Some(file_id))
            .map(|(i, _)| i)
            .collect();

        if file_positions.is_empty() {
            continue;
        }

        let mut current_space = 0;
        let mut space_start = 0;
        let mut found_space = None;

        for (i, &pos) in disk.iter().enumerate() {
            if pos.is_none() {
                if current_space == 0 {
                    space_start = i;
                }
                current_space += 1;
                if current_space == file_size {
                    found_space = Some(space_start);
                    break;
                }
            } else {
                current_space = 0;
            }
        }

        if let Some(new_start) = found_space {
            if new_start < file_positions[0] {
                // Clear old positions
                for pos in file_positions {
                    disk[pos] = None;
                }
                // Move file to new position
                for i in 0..file_size {
                    disk[new_start + i] = Some(file_id);
                }
            }
        }
    }

    for (i, num) in disk.iter().enumerate() {
        if let Some(file_id) = num {
            sum += i as u64 * *file_id as u64;
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
