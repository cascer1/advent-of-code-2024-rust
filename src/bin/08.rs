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
    let (map, width, height) = parse_input(input);
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennae) in map.iter() {
        antinodes.extend(antennae);
        for permutation in antennae.iter().permutations(2) {
            let first = permutation[0];
            let second = permutation[1];

            let dx = second.0 - first.0;
            let dy = second.1 - first.1;

            // Continue iterating over dy/dx until outside the map, both increasing and decreasing
            let mut outside = false;
            let mut i = 0;

            while !outside {
                let nx_positive = first.0 + dx * i;
                let ny_positive = first.1 + dy * i;

                let nx_negative = first.0 - dx * i;
                let ny_negative = first.1 - dy * i;

                let mut positive_inserted = false;
                let mut negative_inserted = false;

                if nx_positive < width && nx_positive >= 0 && ny_positive < height && ny_positive >= 0 {
                    antinodes.insert((nx_positive, ny_positive));
                    positive_inserted = true;
                }

                if nx_negative < width && nx_negative >= 0 && ny_negative < height && ny_negative >= 0 {
                    antinodes.insert((nx_negative, ny_negative));
                    negative_inserted = true;
                }

                if !positive_inserted && !negative_inserted {
                    outside = true;
                }

                i += 1;
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if antinodes.contains(&(x, y)) {
                print!("#")
            } else { print!(".") }
        }
        println!();
    }

    println!("{} unique frequencies", map.keys().count());

    Some(antinodes.len() as u32)
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
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
        // 1003 too high
        // 961 too low
    }

    #[test]
    fn test_part_two_second() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(9))
    }
}
