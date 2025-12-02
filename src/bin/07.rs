advent_of_code::solution!(7);

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn apply(&self, left: u64, right: u64) -> u64 {
        match self {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
            Operator::Concatenate => format!("{}{}", left, right).parse().unwrap(),
        }
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let operators = Vec::from([Operator::Add, Operator::Multiply]);

        if has_valid_solution(target, &numbers, &operators) {
            sum += target;
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut sum: u64 = 0;
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        let target: u64 = parts[0].trim().parse().unwrap();
        let numbers: Vec<u64> = parts[1]
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();

        let operators = Vec::from([Operator::Add, Operator::Multiply, Operator::Concatenate]);

        if has_valid_solution(target, &numbers, &operators) {
            sum += target;
        }
    }

    Some(sum)
}

fn has_valid_solution(expected: u64, numbers: &[u64], operators: &[Operator]) -> bool {
    if operators.is_empty() {
        panic!("Expected at least one operator")
    }

    if numbers.len() < 2 {
        panic!("Need at least two numbers to calculate")
    }

    let num_positions = numbers.len() - 1;
    let num_operators = operators.len();
    let total_combinations = num_operators.pow(num_positions as u32);

    for combination_index in 0..total_combinations {
        let operators = index_to_operators(combination_index, num_positions, operators);
        let result = evaluate_expression(numbers, &operators);
        if result == expected {
            return true
        }
    }

    false
}

fn index_to_operators(
    mut index: usize,
    num_positions: usize,
    available_operators: &[Operator],
) -> Vec<Operator> {
    let base = available_operators.len();
    let mut operators = Vec::with_capacity(num_positions);

    for _ in 0..num_positions {
        operators.push(available_operators[index % base]);
        index /= base;
    }

    operators
}

fn evaluate_expression(numbers: &[u64], operators: &[Operator]) -> u64 {
    let mut result = numbers[0];

    for (i, operator) in operators.iter().enumerate() {
        result = operator.apply(result, numbers[i + 1]);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
