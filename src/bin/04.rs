use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

const TEST2: &str = "\
.M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn create_grid<R: BufRead>(reader: R) -> Vec<Vec<char>> {
        let mut grid: Vec<Vec<char>> = Vec::new();
        for line in reader.lines() {
            let mut row: Vec<char> = Vec::new();
            for c in line.unwrap().chars() {
                row.push(c);
            }
            grid.push(row);
        }
        grid
    }

    fn check_word(word: &str, grid: &Vec<Vec<char>>, row_pos: usize, col_pos: usize, d_row: i32, d_col: i32) -> bool {
        let mut row_ind = row_pos as i32;
        let mut col_ind = col_pos as i32;

        for c in word.chars() {
            if row_ind < 0 || row_ind >= grid.len() as i32  || col_ind < 0 || col_ind >= grid[row_ind as usize].len() as i32 {
                return false;
            }

            if grid[row_ind as usize][col_ind as usize] != c {
                return false;
            }

            row_ind += d_row;
            col_ind += d_col;
        }

        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let grid = create_grid(reader);
        let find_str = "XMAS";
        let mut words_found = 0;

        let directions: Vec<(i32, i32)> = vec![
            (1, 0),     // right
            (-1, 0),    // left
            (0, 1),     // down
            (0, -1),    // up
            (1, -1),    // right up
            (1, 1),     // right down
            (-1, -1),   // left up
            (-1, 1)     // left down
        ];

        for row_ind in 0..grid.len() {
            for col_ind in 0..grid[row_ind].len() {

                for (d_row, d_col) in &directions {
                    if check_word(find_str, &grid, row_ind, col_ind, *d_row, *d_col) {
                        words_found += 1;
                    }
                }
            }
        }

        Ok(words_found)
    }

    assert_eq!(18, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn check_pattern(grid: &Vec<Vec<char>>, row_pos: usize, col_pos: usize) -> bool {
        /*
            M.S     M.M     S.M     S.S
            .A.     .A.     .A.     .A.
            M.S     S.S     S.M     M.M
         */

        if row_pos < 1 || row_pos >= grid.len() - 1  || col_pos < 1 || col_pos >= grid[row_pos].len() -1 {
            return false;
        }

        let lu = grid[row_pos-1][col_pos-1];
        let ru = grid[row_pos+1][col_pos-1];
        let ld = grid[row_pos-1][col_pos+1];
        let rd = grid[row_pos+1][col_pos+1];

        match lu {
            'M'|'S' => {
                match rd {
                    'M'|'S' => { if lu == rd { return false; } }
                    _ => { return false; }
                }
            }
            _ => { return false; }
        }

        match ru {
            'M'|'S' => {
                match ld {
                    'M'|'S' => { if ru == ld { return false; } }
                    _ => { return false; }
                }
            }
            _ => { return false; }
        }

        true
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let grid = create_grid(reader);
        let mut words_found = 0;

        for row_ind in 0..grid.len() {
            for col_ind in 0..grid[row_ind].len() {

                if grid[row_ind][col_ind] == 'A' {
                    if check_pattern(&grid,row_ind,col_ind) {
                        words_found += 1;
                    }
                }

            }
        }

        Ok(words_found)
    }

    assert_eq!(9, part2(BufReader::new(TEST2.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
