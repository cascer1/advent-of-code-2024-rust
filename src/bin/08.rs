extern crate core;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    let (map, width, height) = parse_input(input);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennae) in map.iter() {
        for permutation in antennae.iter().permutations(2) {
            let first = permutation[0];
            let second = permutation[1];

            let dx = second.0 - first.0;
            let dy = second.1 - first.1;

            let antinode_one = (first.0 - dx, first.1 - dy);
            let antinode_two = (second.0 + dx, second.1 + dy);

            if antinode_one.0 >= 0 && antinode_one.0 < width && antinode_one.1 >= 0 && antinode_one.1 < height {
                antinodes.insert(antinode_one);
            }

            if antinode_two.0 >= 0 && antinode_two.0 < width && antinode_two.1 >= 0 && antinode_two.1 < height {
                antinodes.insert(antinode_two);
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse_input(input: &str) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let mut map = HashMap::new();
    let mut width: i32 = 0;
    let mut height = 0;

    for (y, line) in input.lines().enumerate() {
        height += 1;
        width = width.max(line.len() as i32);
        for (x, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            map.entry(c)
                .or_insert_with(Vec::new)
                .push((x as i32, y as i32));
        }
    }

    (map, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
        // 262 too low
        // 303 too high
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
