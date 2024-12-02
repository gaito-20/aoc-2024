use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::enumerate;
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

    enum ReportState {
        SAFE,
        UNSAFE,
    }

    fn check_report(report: Vec<i32>) -> ReportState {
        let mut direction = LevelsDirection::UNSET;
        let mut prev: Option<i32> = None;
        for val in report {
            if prev != None {
                let diff = prev.unwrap() - val;
                if diff.abs() < 1 || diff.abs() > 3 {
                    return ReportState::UNSAFE;
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
                            return ReportState::UNSAFE;
                        }
                    }
                    LevelsDirection::DECREASING => {
                        if diff > 0 {
                            return ReportState::UNSAFE;
                        }
                    }
                }
            }
            prev = Option::from(val);
        }
        ReportState::SAFE
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_counter = 0;

        for line in reader.lines() {
            let line = line?;

            let mut report: Vec<i32> = Vec::new();
            for elem in line.split_whitespace() {
                report.push(elem.parse::<i32>()?);
            }

            match check_report(report) {
                ReportState::SAFE => {
                    safe_counter += 1;
                }
                ReportState::UNSAFE => {}
            }
        }
        Ok(safe_counter)
    }

    assert_eq!(2, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let mut safe_counter = 0;

        for line in reader.lines() {
            let line = line?;

            let mut report: Vec<i32> = Vec::new();
            for elem in line.split_whitespace() {
                report.push(elem.parse::<i32>()?);
            }

            let mut reports_to_check: Vec<Vec<i32>> = Vec::new();
            reports_to_check.push(report.to_vec());

            for (ind, _) in enumerate(report.to_vec()) {
                let mut tolerant_report = report.to_vec();
                tolerant_report.remove(ind);

                reports_to_check.push(tolerant_report);
            }

            let count_safe_reports = reports_to_check
                .iter()
                .map(|x| check_report(x.to_vec()))
                .filter(|x| match x {
                    ReportState::SAFE => true,
                    ReportState::UNSAFE => false,
                })
                .count();

            if count_safe_reports != 0 {
                safe_counter += 1;
            }
        }
        Ok(safe_counter)
    }

    assert_eq!(4, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
