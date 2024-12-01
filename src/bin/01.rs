use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use adv_code_2024::*;

const DAY: &str = "01";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn pop_minimum(list: &mut Vec<i32>) -> i32 {
        let mut min_index: usize = 0;
        let mut minimum: &i32 = list.first().unwrap();
        for (ind, elem) in list.iter().enumerate() {
            if elem < minimum {
                min_index = ind;
                minimum = elem;
            }
        }
        list.remove(min_index)
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {

        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let line = line?;

            let mut iter = line.split_whitespace();
            left.push(iter.next().unwrap().parse::<i32>()?);
            right.push(iter.next().unwrap().parse::<i32>()?);
        }

        assert_eq!(left.len(), right.len());
        
        let mut distance = 0;
        
        while !left.is_empty() {
            let left_min = pop_minimum(&mut left);
            let right_min = pop_minimum(&mut right);
            
            distance += (left_min - right_min).abs();
        }

        Ok(distance as usize)
    }

    assert_eq!(11, part1(BufReader::new(TEST.as_bytes()))?);

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
