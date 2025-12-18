use itertools::Itertools;
use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let (mut occupancy, mut queue) = parse_part_one(input);

    let mut checksum = 0u64;
    let mut index = 0u64;

    while !queue.is_empty() {
        let filled = occupancy.pop_front().unwrap();
        let id = if filled {
            queue.pop_front().unwrap() as u64
        } else {
            queue.pop_back().unwrap() as u64
        };
        checksum += index * id;
        index += 1;
    }

    Some(checksum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (mut disk, queue) = parse_part_two(input);

    for (file_id, length) in queue {
        let new_index = find_empty_index_with_size(&disk, length);
        let old_index = find_file_index(&disk, file_id).expect("File must already be present!");

        if let Some((index, remainder)) = new_index {
            if old_index < index {
                // we're not going to move to the right
                continue;
            }

            disk[old_index] = (0, length, false);
            disk[index] = (file_id, length, true);
            if remainder > 0 {
                disk.insert(index + 1, (0, remainder, false));
            }
        }
    }

    let mut checksum = 0;
    let mut index = 0;

    for (file_id, length, _occupied) in disk {
        for offset in 0..length {
            checksum += file_id * (offset + index)
        }
        index += length
    }

    Some(checksum as u64)
}

fn find_empty_index_with_size(
    disk: &[(usize, usize, bool)],
    length: usize,
) -> Option<(usize, usize)> {
    for (index, space) in disk.iter().enumerate() {
        if space.2 {
            // already occupied
            continue;
        }

        if space.1 >= length {
            return Some((index, space.1 - length));
        }
    }

    None
}

fn find_file_index(disk: &[(usize, usize, bool)], file_id: usize) -> Option<usize> {
    let result = disk.iter().find_position(|file| file.0 == file_id);

    if let Some(data) = result {
        return Some(data.0);
    }

    None
}

fn parse_part_one(input: &str) -> (VecDeque<bool>, VecDeque<usize>) {
    // input alternates with (length occupied), (length empty)
    let mut occupancy = VecDeque::new();
    let mut reverse_queue = VecDeque::new();

    let mut filled = true;
    let mut file_id = 0;

    input.trim().bytes().for_each(|c| {
        let number = c - 0x30;
        if filled {
            for _count in 0..number {
                occupancy.push_back(true);
                reverse_queue.push_back(file_id);
            }
            file_id += 1
        } else {
            for _count in 0..number {
                occupancy.push_back(false);
            }
        }
        filled = !filled;
    });

    (occupancy, reverse_queue)
}

fn parse_part_two(input: &str) -> (Vec<(usize, usize, bool)>, VecDeque<(usize, usize)>) {
    let mut disk = Vec::new();
    let mut reverse_queue = VecDeque::new();

    let mut filled = true;
    let mut file_id: usize = 0;
    input.trim().bytes().for_each(|c| {
        let number = c as usize - 0x30;

        if filled {
            disk.push((file_id, number, true));
            reverse_queue.push_front((file_id, number));
            file_id += 1;
        } else {
            disk.push((0, number, false));
        }
        filled = !filled;
    });

    (disk, reverse_queue)
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
    fn test_part_one_known_wrong() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_ne!(result, Some(4162404088));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
