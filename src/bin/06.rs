use std::result::Result::Ok;
use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{enumerate};
use adv_code_2024::*;

const DAY: &str = "06";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn create_map<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        let mut map: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.unwrap().chars() {
                row.push(c);
            }
            map.push(row);
        }
        map
    }

    #[derive(PartialEq)]
    enum Direction {
        Up,
        Down,
        Left,
        Right
    }

    fn get_pos(map: &Vec<Vec<char>>) -> Result<(usize, usize, Direction)> {
        for (row_ind, row) in enumerate(map) {
            for (c_ind, &c) in enumerate(row) {
                match c {
                    '^' => { return Ok((row_ind, c_ind, Direction::Up)); }
                    '>' => { return Ok((row_ind, c_ind, Direction::Right)); }
                    'v' => { return Ok((row_ind, c_ind, Direction::Down)); }
                    '<' => { return Ok((row_ind, c_ind, Direction::Left)); }
                    _ => {}
                }
            }
        }
        panic!("Guard not found");
    }

    /**
    * Processes the next move. Returns true if guard left the map
    */
    fn next_move(map: &mut Vec<Vec<char>>) -> bool {
        let (pos_row, pos_col, direction) = get_pos(&map).unwrap();

        let mut d_row: i32 = 0;
        let mut d_col: i32 = 0;

        match direction {
            Direction::Up => { d_row = -1; }
            Direction::Down => { d_row = 1; }
            Direction::Left => { d_col = -1; }
            Direction::Right => { d_col = 1; }
        }

        let (next_pos_row, next_pos_col) = (pos_row as i32 + d_row, pos_col as i32 + d_col);

        let row_size = map.len();
        let col_size = map.get(0).unwrap().len();

        if next_pos_row < 0 || next_pos_row > (row_size-1) as i32 || next_pos_col < 0 || next_pos_col > (col_size-1) as i32 {
            *map.get_mut(pos_row).unwrap().get_mut(pos_col).unwrap() = 'X';
            return true;
        }

        let next_char = *map.get(next_pos_row as usize).unwrap().get(next_pos_col as usize).unwrap();
        let curr_char = map.get_mut(pos_row).unwrap().get_mut(pos_col).unwrap();

        let mut set_next_char = '.';

        match next_char {
            '#' => {
                set_next_char = '#';
                match direction {
                    Direction::Up => {
                        // next direction = right
                        *curr_char = '>';
                    }
                    Direction::Down => {
                        // next direction = left
                        *curr_char = '<';
                    }
                    Direction::Left => {
                        // next direction = up
                        *curr_char = '^';
                    }
                    Direction::Right => {
                        // next direction = down
                        *curr_char = 'v';
                    }
                }
            }
            '.' | 'X' => {
                *curr_char = 'X';
                set_next_char = match direction {
                    Direction::Up => { '^' }
                    Direction::Down => { 'v' }
                    Direction::Left => { '<' }
                    Direction::Right => { '>' }
                }
            }
            _ => {}
        }

        *map.get_mut(next_pos_row as usize).unwrap().get_mut(next_pos_col as usize).unwrap() = set_next_char;
        false
    }


    fn count_visits(map: &Vec<Vec<char>>) -> usize {
        let mut counter = 0;
        for row in map {
            for &c in row {
                if c == 'X' {
                    counter += 1;
                }
            }
        }
        counter
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = create_map(reader);
        while !next_move(&mut map) { }
        Ok(count_visits(&map))
    }

    assert_eq!(41, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn search(direction: Direction, map: &Vec<Vec<char>>, remember_direction: &mut Vec<Vec<Option<Direction>>>, pos_row: usize, pos_col: usize) -> bool {
        let mut d_row: i32 = 0;
        let mut d_col: i32 = 0;

        match direction {
            Direction::Up => { d_row = -1; }
            Direction::Down => { d_row = 1; }
            Direction::Left => { d_col = -1; }
            Direction::Right => { d_col = 1; }
        }
        
        let (next_pos_row, next_pos_col) = (pos_row as i32 + d_row, pos_col as i32 + d_col);

        let row_size = map.len();
        let col_size = map.get(0).unwrap().len();

        if next_pos_row < 0 || next_pos_row > (row_size-1) as i32 || next_pos_row < 0 || next_pos_col > (col_size-1) as i32 {
            return false;
        }
        
        let next_char = map.get(next_pos_row as usize).unwrap().get(next_pos_col as usize).unwrap();
        let next_direction = remember_direction.get(next_pos_row as usize).unwrap().get(next_pos_col as usize).unwrap();

        match next_direction {
            None => {}
            Some(next_dir) => {
                if *next_dir == direction {
                    return true;
                } else {
                    return false;
                }
            }
        }
        
        match next_char {
            '#' => { return false; }
            _ => {  }
        }
        
        search(direction, map, remember_direction, pos_row, pos_col)
    }
    
    /**
    * Looks if a path to a loop is possible from the current position, returns true if so.
    */
    fn search_for_continuing_trail(map: &mut Vec<Vec<char>>, remember_direction: &mut Vec<Vec<Option<Direction>>>, pos_row: usize, pos_col: usize, cur_direction: Direction) -> bool {
        match cur_direction {
            Direction::Up => {
                search(Direction::Down, map, remember_direction, pos_row, pos_col) || search(Direction::Left, map, remember_direction, pos_row, pos_col) || search(Direction::Right, map, remember_direction, pos_row, pos_col)
            }
            Direction::Down => {
                search(Direction::Up, map, remember_direction, pos_row, pos_col) || search(Direction::Left, map, remember_direction, pos_row, pos_col) || search(Direction::Right, map, remember_direction, pos_row, pos_col)
            }
            Direction::Left => {
                search(Direction::Down, map, remember_direction, pos_row, pos_col) || search(Direction::Up, map, remember_direction, pos_row, pos_col) || search(Direction::Right, map, remember_direction, pos_row, pos_col)
            }
            Direction::Right => {
                search(Direction::Down, map, remember_direction, pos_row, pos_col) || search(Direction::Left, map, remember_direction, pos_row, pos_col) || search(Direction::Up, map, remember_direction, pos_row, pos_col)
            }
        }
    }


    fn next_move2(map: &mut Vec<Vec<char>>, counter: &mut usize, remember_direction: &mut Vec<Vec<Option<Direction>>>) -> bool {
        let (pos_row, pos_col, direction) = get_pos(&map).unwrap();
        *remember_direction.get_mut(pos_row).unwrap().get_mut(pos_col).unwrap() = match direction {
            Direction::Up => { Some(Direction::Up) }
            Direction::Down => { Some(Direction::Down) }
            Direction::Left => { Some(Direction::Left) }
            Direction::Right => { Some(Direction::Right) }
        };

        let mut d_row: i32 = 0;
        let mut d_col: i32 = 0;

        match direction {
            Direction::Up => { d_row = -1; }
            Direction::Down => { d_row = 1; }
            Direction::Left => { d_col = -1; }
            Direction::Right => { d_col = 1; }
        }

        let (next_pos_row, next_pos_col) = (pos_row as i32 + d_row, pos_col as i32 + d_col);

        let row_size = map.len();
        let col_size = map.get(0).unwrap().len();

        if next_pos_row < 0 || next_pos_row > (row_size-1) as i32 || next_pos_col < 0 || next_pos_col > (col_size-1) as i32 {
            *map.get_mut(pos_row).unwrap().get_mut(pos_col).unwrap() = 'X';
            return true;
        }

        let next_char = *map.get(next_pos_row as usize).unwrap().get(next_pos_col as usize).unwrap();
        let curr_char = map.get_mut(pos_row).unwrap().get_mut(pos_col).unwrap();

        let mut set_next_char = '.';

        match next_char {
            '#' => {
                set_next_char = '#';
                match direction {
                    Direction::Up => {
                        // next direction = right
                        *curr_char = '>';
                    }
                    Direction::Down => {
                        // next direction = left
                        *curr_char = '<';
                    }
                    Direction::Left => {
                        // next direction = up
                        *curr_char = '^';
                    }
                    Direction::Right => {
                        // next direction = down
                        *curr_char = 'v';
                    }
                }
            }
            '.' | 'X' => {
                *curr_char = 'X';
                set_next_char = match direction {
                    Direction::Up => { '^' }
                    Direction::Down => { 'v' }
                    Direction::Left => { '<' }
                    Direction::Right => { '>' }
                };

                if next_char == 'X' {
                    match remember_direction.get(next_pos_row as usize).unwrap().get(next_pos_col as usize).unwrap() {
                        None => {}
                        Some(next_direction) => {
                            match next_direction {
                                Direction::Up => {
                                    match direction {
                                        Direction::Left => { *counter += 1; }
                                        _ => {}
                                    }
                                }
                                Direction::Down => {
                                    match direction {
                                        Direction::Right => { *counter += 1; }
                                        _ => {}
                                    }
                                }
                                Direction::Left => {
                                    match direction {
                                        Direction::Down => { *counter += 1; }
                                        _ => {}
                                    }
                                }
                                Direction::Right => {
                                    match direction {
                                        Direction::Up => { *counter += 1; }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }
                } else {
                    if search_for_continuing_trail(map, remember_direction, pos_row, pos_col, direction) {
                        *counter += 1;
                    }
                }
            }
            _ => {}
        }

        *map.get_mut(next_pos_row as usize).unwrap().get_mut(next_pos_col as usize).unwrap() = set_next_char;
        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut map = create_map(reader);

        let mut remember_direction: Vec<Vec<Option<Direction>>> = Vec::new();
        for row in &map {
            let mut remember_direction_row: Vec<Option<Direction>> = Vec::new();
            for _ in row {
                remember_direction_row.push(None);
            }
            remember_direction.push(remember_direction_row);
        }

        let mut counter: usize = 0;
        while !next_move2(&mut map, &mut counter, &mut remember_direction) { }
        Ok(counter)
    }

    assert_eq!(6, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
