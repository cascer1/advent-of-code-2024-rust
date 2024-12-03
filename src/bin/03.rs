use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let pattern = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let matches = pattern.captures_iter(input);

    Some(
        matches
            .map(|capture| {
                capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                    * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut enabled = true;
    let mut total = 0;

    let pattern = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let captures = pattern.captures_iter(input);
    
    captures.for_each(|capture| {
        let complete = capture.get(0).unwrap().as_str();
        
        if complete == "do()" {
            enabled = true;
        } else if complete == "don't()" {
            enabled = false;
        } else if enabled {
            let left = capture.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let right = capture.get(2).unwrap().as_str().parse::<u32>().unwrap();
            total += left * right;
        }
    });

    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
