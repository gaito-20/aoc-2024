use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    enum LevelsDirection {
        UNSET,
        INCREASING,
        DECREASING,
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_counter = 0;
        
        'report: for line in reader.lines() {
            let line = line?;
            let mut direction = LevelsDirection::UNSET;
            let mut prev: Option<i32> = None;
            
            for elem in line.split_whitespace() {
                let val = Option::from(elem.parse::<i32>()?);

                if prev != None {
                    let diff = prev.unwrap() - val.unwrap();
                    if diff.abs() < 1 || diff.abs() > 3 {
                        continue 'report;
                    }
                    match direction {
                        LevelsDirection::UNSET => {
                            if diff < 0 {
                                direction = LevelsDirection::DECREASING;
                            } else {
                                direction = LevelsDirection::INCREASING;
                            }
                        }
                        LevelsDirection::INCREASING => {
                            if diff < 0 {
                                continue 'report;
                            }
                        }
                        LevelsDirection::DECREASING => {
                            if diff > 0 {
                                continue 'report;
                            }
                        }
                    }
                }
                prev = val;
            }
            safe_counter += 1;
        }
        Ok(safe_counter)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

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
