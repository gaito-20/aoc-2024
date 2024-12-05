use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::{enumerate};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "05";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn create_rules(input: &Vec<String>) -> HashMap<i32, Vec<i32>> {
        let mut rules: HashMap<i32, Vec<i32>> = HashMap::new();
        for line in input {
            let x: Vec<&str> = line.split("|").collect();
            let page = x[0].parse::<i32>().unwrap();
            let page_rule = x[1].parse::<i32>().unwrap();

            let entry = rules.get(&page);
            match entry {
                None => { rules.insert(page, vec![page_rule]); }
                Some(elem) => {
                    let mut new_rules = elem.to_vec();
                    new_rules.push(page_rule);
                    rules.insert(page, new_rules);
                }
            }
        }
        rules
    }

    fn create_updates(input: &Vec<String>) -> Vec<Vec<i32>> {
        let mut updates_list: Vec<Vec<i32>> = Vec::new();
        for line in input {

            let mut updates: Vec<i32> = Vec::new();
            let updates_str: Vec<&str> = line.split(",").collect();

            for update_str in updates_str {
                match update_str.parse::<i32>() {
                    Result::Ok(update) => {updates.push(update)}
                    Err(_) => {}
                }
            }

            updates_list.push(updates);
        }
        updates_list
    }

    fn read_input<R: BufRead>(reader: R) -> Result<(HashMap<i32, Vec<i32>>, Vec<Vec<i32>>)> {
        let mut rules_lines: Vec<String> = Vec::new();
        let mut updates_lines: Vec<String> = Vec::new();
        for line in reader.lines() {
            let line = line?;
            match line {
                l if l.contains("|") => { rules_lines.push(l); }
                l if l.contains(",") => { updates_lines.push(l); }
                _ => {}
            }
        }

        let rules_map = create_rules(&rules_lines);
        let updates_list = create_updates(&updates_lines);

        Ok((rules_map, updates_list))
    }

    fn check_rules(rules: &HashMap<i32, Vec<i32>>, update: &Vec<i32>) -> bool {
        for (ind, page) in enumerate(update) {
            let pages_before= &update[0..ind];

            for earlier_page in pages_before {
                match rules.get(page) {
                    None => {}
                    Some(rule) => { if rule.contains(earlier_page) { return false; } }
                }
            }
        }
        true
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let (rules_map, updates_list) = read_input(reader)?;
        let mut valid_updates: Vec<Vec<i32>> = Vec::new();

        for update in updates_list {
            if check_rules(&rules_map, &update) {
                valid_updates.push(update);
            }
        }

        let mut median_sum = 0;
        for update in valid_updates {
            median_sum += update.get(update.len() / 2).unwrap();
        }
        
        Ok(median_sum as usize)
    }

    assert_eq!(143, part1(BufReader::new(TEST.as_bytes()))?);

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
