use std::collections::VecDeque;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut disk, mut queue) = parse(input);
    // disk: (file_id, filled)
    // queue: file_id

    let block_count = disk.len();
    let mut new_disk: Vec<u8> = Vec::new();

    while new_disk.len() < block_count {
        let (file_id, filled) = disk.pop_front().unwrap();
        if filled {
            new_disk.push(file_id);
        } else {
            new_disk.push(queue.pop_front().unwrap());
        }
    }

    Some(calculate_checksum(new_disk))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn calculate_checksum(disk: Vec<u8>) -> u32 {
    let mut result = 0usize;

    for (index, number) in disk.iter().enumerate() {
        let part = index * *number as usize;
        result += part;
        println!("{} * {} = {}", index, number, part);
    }

    result as u32
}

fn parse(input: &str) -> (VecDeque<(u8, bool)>, VecDeque<u8>) {
    // input alternates with (length occupied), (length empty)
    let mut disk = VecDeque::new();
    let mut reverse_queue = VecDeque::new();

    let mut filled = true;
    let mut file_id = 0;

    input.chars().for_each(|c| {
        let number = c as u8 - 0x30;
        if (filled) {
            for _count in 0..number {
                disk.push_back((file_id, true));
                reverse_queue.push_back(file_id);
            }
            file_id += 1
        } else {
            for _count in 0..number {
                disk.push_back((file_id, true));
            }
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
