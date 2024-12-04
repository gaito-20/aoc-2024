use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::{Rc, Weak};

const DAY: &str = "04";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

#[derive(Eq, Hash, PartialEq)]
enum Direction {
    UP,
    RIGHT_UP,
    RIGHT,
    RIGHT_DOWN,
    DOWN,
    LEFT_DOWN,
    LEFT,
    LEFT_UP,
}

struct Cell {
    content: char,
    neighbors: HashMap<Direction, Weak<RefCell<Cell>>>,
}

impl Cell {
    fn new(content: char) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Cell {
            content,
            neighbors: HashMap::new(),
        }))
    }

    fn link_cells(cell: &Rc<RefCell<Cell>>, other: &Rc<RefCell<Cell>>, direction_other: Direction) {
        let reverse_direction = match direction_other {
            Direction::UP => Direction::DOWN,
            Direction::RIGHT_UP => Direction::LEFT_DOWN,
            Direction::RIGHT => Direction::LEFT,
            Direction::RIGHT_DOWN => Direction::LEFT_UP,
            Direction::DOWN => Direction::UP,
            Direction::LEFT_DOWN => Direction::RIGHT_UP,
            Direction::LEFT => Direction::RIGHT,
            Direction::LEFT_UP => Direction::RIGHT_DOWN,
        };

        cell.borrow_mut()
            .neighbors
            .insert(direction_other, Rc::downgrade(&other));
        other
            .borrow_mut()
            .neighbors
            .insert(reverse_direction, Rc::downgrade(&cell));
    }
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn link_grid(grid: &Vec<Vec<Rc<RefCell<Cell>>>>) {
        let grid_len = grid.clone().len();
        for row_ind in 0..grid_len-1 {
            let row = grid[row_ind].clone();
            let down_row = grid[row_ind+1].clone();
            let row_len = row.clone().len();

            for col_ind in 0..row_len-1 {
                let cell = row[col_ind].clone();
                let down_cell = down_row[col_ind].clone();
                let right_cell = row[col_ind+1].clone();
                let right_down_cell = down_row[col_ind+1].clone();

                if row_ind < grid_len - 1 {
                    Cell::link_cells(&cell, &down_cell, Direction::DOWN);
                }

                if col_ind < row_len - 1{
                    Cell::link_cells(&cell, &right_cell, Direction::RIGHT)
                }

                if row_ind < grid_len - 1 && col_ind < row_len - 1{
                    Cell::link_cells(&cell, &right_down_cell, Direction::RIGHT_DOWN);
                    Cell::link_cells(&down_cell, &right_cell, Direction::RIGHT_UP);
                }
            }
        }

    }

    fn create_grid<R: BufRead>(reader: R) -> Vec<Vec<Rc<RefCell<Cell>>>> {
        let mut grid: Vec<Vec<Rc<RefCell<Cell>>>> = Vec::new();
        for line in reader.lines() {
            let mut row: Vec<Rc<RefCell<Cell>>> = Vec::new();
            for c in line.unwrap().chars() {
                row.push(Cell::new(c));
            }
            grid.push(row);
        }
        link_grid(&grid);
        grid
    }

    fn print_cell(cell: Rc<RefCell<Cell>>) {
        fn get_neighbour_char(cell: &Rc<RefCell<Cell>>, direction: Direction) -> char {
            match cell.borrow().neighbors.get(&direction) {
                None => ' ',
                Some(cell) => {
                    match cell.upgrade() {
                        None => ' ',
                        Some(cell) => cell.borrow().content
                    }
                }
            }
        }

        println!("+---+---+---+");
        println!("| {} | {} | {} |", get_neighbour_char(&cell.clone(), Direction::LEFT_UP), get_neighbour_char(&cell.clone(), Direction::UP), get_neighbour_char(&cell.clone(), Direction::RIGHT_UP));
        println!("+---+---+---+");
        println!("| {} | {} | {} |", get_neighbour_char(&cell.clone(), Direction::LEFT), cell.borrow().content, get_neighbour_char(&cell.clone(), Direction::RIGHT));
        println!("+---+---+---+");
        println!("| {} | {} | {} |", get_neighbour_char(&cell.clone(), Direction::LEFT_DOWN), get_neighbour_char(&cell.clone(), Direction::DOWN), get_neighbour_char(&cell.clone(), Direction::RIGHT_DOWN));
        println!("+---+---+---+");

    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = create_grid(reader);

        for row in grid {
            for cell in row {
                print_cell(cell);
                println!();
            }
            println!("=============")
        }

        Ok(0)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    // println!("\n=== Part 2 ===");
    //
    // fn part2<R: BufRead>(reader: R) -> Result<usize> {
    //     Ok(0)
    // }
    //
    // assert_eq!(0, part2(BufReader::new(TEST.as_bytes()))?);
    //
    // let input_file = BufReader::new(File::open(INPUT_FILE)?);
    // let result = time_snippet!(part2(input_file)?);
    // println!("Result = {}", result);
    //endregion

    Ok(())
}
