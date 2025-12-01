use std::collections::hash_map::Entry;
use std::collections::HashMap;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules, print_orders) = parse(input);

    Some(
        print_orders
            .iter()
            .filter(|order| print_order_is_correct(order, &rules))
            .map(|order| {
                let len = order.len();
                let middle_index = len / 2;
                order[middle_index]
            })
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules, print_orders) = parse(input);

    Some(
        print_orders
            .iter()
            .filter(|order| !print_order_is_correct(order, &rules))
            .map(|order| fix_print_order(order, &rules))
            .map(|order| {
                let len = order.len();
                let middle_index = len / 2;
                order[middle_index]
            })
            .sum(),
    )
}

fn parse(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let mut rules = HashMap::new();
    let mut print_orders: Vec<Vec<u32>> = Vec::new();

    input.lines().for_each(|line| {
        if line.contains('|') {
            add_rule(line, &mut rules);
        } else if line.contains(',') {
            print_orders.push(parse_as_print_order(line))
        }
    });

    (rules, print_orders)
}

fn add_rule(line: &str, rules: &mut HashMap<u32, Vec<u32>>) {
    let (left, right) = line.split_once('|').unwrap();
    let left_page: u32 = left.parse().unwrap();
    let right_page: u32 = right.parse().unwrap();

    if let Entry::Vacant(e) = rules.entry(right_page) {
        e.insert(vec![left_page]);
    } else {
        rules.get_mut(&right_page).unwrap().push(left_page);
    }
}

fn parse_as_print_order(line: &str) -> Vec<u32> {
    line.split(',').map(|s| s.parse().unwrap()).collect()
}

fn print_order_is_correct(print_order: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    let mut printed_pages: Vec<u32> = Vec::new();

    for page in print_order {
        let required_pages = rules.get(page);

        if required_pages.is_some()
            && !required_pages
                .unwrap()
                .iter()
                .all(|page| printed_pages.contains(page) || !print_order.contains(page))
        {
            return false;
        }

        printed_pages.push(*page);
    }

    true
}

fn fix_print_order(
    remaining_pages: &Vec<u32>,
    original_rules: &HashMap<u32, Vec<u32>>,
) -> Vec<u32> {
    fix_print_order_recursive(remaining_pages, original_rules, Vec::new())
}

fn fix_print_order_recursive(
    remaining_pages: &Vec<u32>,
    original_rules: &HashMap<u32, Vec<u32>>,
    mut intermediate_job: Vec<u32>,
) -> Vec<u32> {
    // Base case: nothing left to schedule
    if remaining_pages.is_empty() {
        return intermediate_job;
    }

    // Find the next page whose prerequisites are either already printed
    // or not present in the remaining pages (thus can be ignored).
    let mut next_page: Option<u32> = None;
    let empty: Vec<u32> = Vec::new();

    'outer: for page in remaining_pages.iter() {
        let prerequisites = original_rules.get(page).unwrap_or(&empty);

        for prerequisite in prerequisites.iter() {
            // If a prerequisite is still remaining and not yet printed, this page cannot be printed now.
            if remaining_pages.contains(prerequisite) && !intermediate_job.contains(prerequisite) {
                continue 'outer;
            }
        }
        next_page = Some(*page);
        break;
    }

    let next_page = next_page.expect("Could not find next page to print!");

    // Append selected page and continue recursively
    intermediate_job.push(next_page);

    let new_remaining_pages: Vec<u32> = remaining_pages
        .iter()
        .copied()
        .filter(|page| *page != next_page)
        .collect();

    fix_print_order_recursive(&new_remaining_pages, original_rules, intermediate_job)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
