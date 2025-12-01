use crate::Dir::{East, North, South, West};
use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut tiles: HashSet<Pos> = HashSet::new();
    let mut visited: HashSet<Pos> = HashSet::new();
    let mut pos = Pos { x: 0, y: 0 };
    let mut dir = North;

    let mut x = 0;
    let mut y = 0;

    input.lines().for_each(|line| {
        x = 0;
        width = line.len() as i32;
        height += 1;
        line.chars().for_each(|c| {
            if c == '#' {
                tiles.insert(Pos { x, y });
            } else if c == '^' {
                pos = Pos { x, y };
            }

            x += 1
        });

        y += 1;
    });

    while pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height {
        visited.insert(pos);
        let mut next_pos = step(dir, pos);

        while tiles.contains(&next_pos) {
            dir = rotate(dir);
            next_pos = step(dir, pos)
        }

        pos = next_pos;
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Parse grid once
    let mut width: i32 = 0;
    let mut height: i32 = 0;
    let mut tiles: HashSet<Pos> = HashSet::new();
    let mut start = Pos { x: 0, y: 0 };
    let start_dir = North;

    let mut x = 0;
    let mut y = 0;
    input.lines().for_each(|line| {
        x = 0;
        width = line.len() as i32;
        height += 1;
        line.chars().for_each(|c| {
            if c == '#' {
                tiles.insert(Pos { x, y });
            } else if c == '^' {
                start = Pos { x, y };
            }
            x += 1;
        });
        y += 1;
    });

    // Part 1 traversal to collect visited cells as candidate positions
    let mut visited_cells: HashSet<Pos> = HashSet::new();
    {
        let mut pos = start;
        let mut dir = start_dir;
        while pos.x >= 0 && pos.x < width && pos.y >= 0 && pos.y < height {
            visited_cells.insert(pos);
            let mut next_pos = step(dir, pos);
            while tiles.contains(&next_pos) {
                dir = rotate(dir);
                next_pos = step(dir, pos);
            }
            pos = next_pos;
        }
    }

    // Closure to determine whether adding a block at `block` causes a loop
    let mut causes_loop = |block: Pos| -> bool {
        if tiles.contains(&block) || block == start { return false; }
        let mut pos = start;
        let mut dir = start_dir;
        let mut seen: HashSet<(Pos, Dir)> = HashSet::new();
        loop {
            // out of bounds => no loop
            if pos.x < 0 || pos.x >= width || pos.y < 0 || pos.y >= height { return false; }
            // state repetition => loop
            if !seen.insert((pos, dir)) { return true; }

            // attempt to step; rotate while blocked by existing or the new block
            let mut next_pos = step(dir, pos);
            while tiles.contains(&next_pos) || next_pos == block {
                dir = rotate(dir);
                next_pos = step(dir, pos);
            }
            pos = next_pos;
        }
    };

    // Only positions on the path (except the start) can influence into a loop
    let count = visited_cells
        .iter()
        .filter(|&&p| p != start)
        .filter(|&&p| causes_loop(p))
        .count() as u32;

    Some(count)
}

fn step(direction: Dir, position: Pos) -> Pos {
    if direction == North {
        Pos {
            x: position.x,
            y: position.y - 1,
        }
    } else if direction == East {
        Pos {
            x: position.x + 1,
            y: position.y,
        }
    } else if direction == South {
        Pos {
            x: position.x,
            y: position.y + 1,
        }
    } else {
        Pos {
            x: position.x - 1,
            y: position.y,
        }
    }
}

fn rotate(direction: Dir) -> Dir {
    match direction {
        North => East,
        East => South,
        South => West,
        West => North,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Pos {
    x: i32,
    y: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
