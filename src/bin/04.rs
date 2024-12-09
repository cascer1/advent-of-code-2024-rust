advent_of_code::solution!(4);

use grid::*;

pub fn part_one(input: &str) -> Option<u32> {
    let grid = parse_input(input);
    let word = "XMAS";

    let mut word_count: u32 = 0;

    grid.indexed_iter().for_each(|((row, col), _)| {
        word_count += get_word_start_count_at(&grid, row as i16, col as i16, word);
    });

    Some(word_count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = parse_input(input);

    let mut word_count: u32 = 0;

    grid.indexed_iter().for_each(|((row, col), _)| {
        word_count += get_cross_count_at(&grid, row as i16, col as i16);
    });

    Some(word_count)
}

fn parse_input(input: &str) -> Grid<char> {
    let line_length = input.lines().next().unwrap().len();
    let input_chars = input.replace('\n', "").chars().collect::<Vec<_>>();

    Grid::from_vec(input_chars, line_length)
}

fn get_word_start_count_at(grid: &Grid<char>, col: i16, row: i16, word: &str) -> u32 {
    // First check if we're at the start of the word
    let chars = &word.chars().collect::<Vec<_>>();

    if grid.get(row, col) != Some(&chars[0]) {
        return 0;
    }

    // left right up down leftup leftdown rightup rightdown
    let horizontal_moves = [-1i16, 1, 0, 0, -1, -1, 1, 1];
    let vertical_moves = [0i16, 0, 1, -1, 1, -1, 1, -1];

    let mut valid_directions = [false; 8];
    let word_length = word.len();

    'directions: for direction in 0..valid_directions.len() {
        let horizontal_step = horizontal_moves[direction];
        let vertical_step = vertical_moves[direction];
        
        for letter in 1..word_length {
            let new_row = row + vertical_step * letter as i16;
            let new_col = col + horizontal_step * letter as i16;

            if new_row < 0 || new_col < 0 {
                continue 'directions;
            }

            let found_letter = grid.get(new_row, new_col);

            if found_letter.is_none() {
                continue 'directions;
            }

            if !chars[letter].eq(found_letter.unwrap()) {
                continue 'directions;
            }
        }

        valid_directions[direction] = true;
    }

    valid_directions.iter().filter(|&&x| x).count() as u32
}

fn get_cross_count_at(grid: &Grid<char>, row: i16, col: i16) -> u32 {
    
    let rows = grid.rows() as i16;
    let cols = grid.cols() as i16;
    
    // We can't be in the middle if we're right at the edge of the grid
    if row == 0 || row == rows || col == 0 || col == cols {
        return 0;
    }
    
    let found_letter = grid.get(row, col).unwrap();
    
    if found_letter != &'A' {
        return 0;
    }
    
    let left_up = grid.get(row - 1, col - 1);
    let left_down = grid.get(row + 1, col - 1);
    let right_up = grid.get(row - 1, col + 1);
    let right_down = grid.get(row + 1, col + 1);
    
    if left_up.is_none() || left_down.is_none() || right_up.is_none() || right_down.is_none() {
        return 0;
    }
    
    let one = left_up.unwrap();
    let two = right_down.unwrap();
    let three = left_down.unwrap();
    let four = right_up.unwrap();
    
    let m = &'M';
    let s = &'S';
    
    if ((one == m && two == s) || (one == s && two == m)) && ((three == m && four == s) || (three == s && four == m)) {
        return 1;
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
