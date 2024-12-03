use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use regex::Regex;
use adv_code_2024::*;

const DAY: &str = "03";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
";

const TEST2: &str = "\
xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let input: String = reader.lines().flatten().collect();
        let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)")?;
        let mul_params = re.captures_iter(&*input).map(|caps| {
            let (_, [x,y]) = caps.extract();
            (x,y)
        });

        let mut res = 0;
        for (x, y) in mul_params {
            res += x.parse::<i32>()? * y.parse::<i32>()?;
        }

        Ok(res as usize)
    }

    assert_eq!(161, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");
    
    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let input: String = reader.lines().flatten().collect();
        let re = Regex::new(r"don't\(\).*?do\(\)")?;
        let cleared_text = re.replace_all(&*input, "");
        
        let re = Regex::new(r"mul\(([\d]{1,3}),([\d]{1,3})\)")?;
        let mul_params = re.captures_iter(&*cleared_text).map(|caps| {
            let (_, [x,y]) = caps.extract();
            (x,y)
        });

        let mut res = 0;
        for (x, y) in mul_params {
            res += x.parse::<i32>()? * y.parse::<i32>()?;
        }

        Ok(res as usize)
    }
    
    assert_eq!(48, part2(BufReader::new(TEST2.as_bytes()))?);
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
