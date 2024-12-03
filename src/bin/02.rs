use crate::ReportMode::{DECREASING, INCREASING};

advent_of_code::solution!(2);

#[derive(PartialEq, Eq, Debug)]
enum ReportMode {
    INCREASING,
    DECREASING,
}

pub fn part_one(input: &str) -> Option<u32> {
    let reports = parse(input);

    Some(
        reports
            .iter()
            .map(|report| is_safe(report))
            .filter(|result| *result == true)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let reports = parse(input);

    Some(
        reports
            .iter()
            .map(|report| is_safe_with_removal(report))
            .filter(|result| *result == true)
            .count() as u32,
    )
}

fn is_safe(report: &Vec<i32>) -> bool {
    let mode: ReportMode;

    let first = report[0];
    let second = report[1];

    if first < second {
        mode = INCREASING;
    } else {
        mode = DECREASING;
    }

    for index in 0..report.len() - 1 {
        let left = report[index];
        let right = report[index + 1];

        // If increasing, each new number must be larger than the previous
        if mode == INCREASING {
            if left >= right {
                return false;
            }

            if right - left > 3 {
                return false;
            }
        }

        // If decreasing, each new number must be smaller than the previous
        if mode == DECREASING {
            if left <= right {
                return false;
            }

            if left - right > 3 {
                return false;
            }
        }
    }

    true
}

fn is_safe_with_removal(report: &Vec<i32>) -> bool {
    for index in 0..report.len() {
        let mut modified_report = report.clone();
        modified_report.remove(index);
        if is_safe(&modified_report) {
            return true;
        }
    }

    false
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
