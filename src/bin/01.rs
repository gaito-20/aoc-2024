use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead, BufReader};

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

    fn read_challenge<R: BufRead>(reader: R) -> (Vec<i32>, Vec<i32>) {
        let mut left: Vec<i32> = Vec::new();
        let mut right: Vec<i32> = Vec::new();

        for line in reader.lines() {
            let line = line.unwrap();

            let mut iter = line.split_whitespace();
            left.push(iter.next().unwrap().parse::<i32>().unwrap());
            right.push(iter.next().unwrap().parse::<i32>().unwrap());
        }
        assert_eq!(left.len(), right.len());

        (left, right)
    }

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
        let (mut left, mut right) = read_challenge(reader);
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
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let (left, right) = read_challenge(reader);

        let right_map = right.iter().counts();
        let mut distance = 0;

        for elem in left {
            distance += (elem as usize) * right_map.get(&elem).unwrap_or(&0);
        }

        Ok(distance)
    }

    assert_eq!(31, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
