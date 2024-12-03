advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left, mut right) = parse(input);

    left.sort();
    right.sort();

    Some(
        left.iter()
            .zip(right.iter())
            .map(|(a, b)| (a - b).abs() as u32)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left, right) = parse(input);

    let mut similarity: u32 = 0;

    for number in left {
        let occurance = right.iter().filter(|&n| *n == number).count() as i32;
        let score = occurance * number;
        similarity += score as u32
    }

    Some(similarity)
}

fn parse(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() == 2 {
            left.push(parts[0].parse::<i32>().unwrap());
            right.push(parts[1].parse::<i32>().unwrap());
        }
    }

    (left, right)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
