use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "07";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    enum Operation {
        Multiply,
        Add,
    }

    fn read_operations<R: BufRead>(reader: R) -> Result<Vec<(u64, Vec<u64>)>> {
        let mut operations: Vec<(u64, Vec<u64>)> = Vec::new();
        for line in reader.lines() {
            let line = line?;

            let x: Vec<&str> = line.split(":").collect();
            let result = x[0].parse::<u64>()?;

            let operands: Vec<u64> = x[1]
                .trim()
                .split_whitespace()
                .map(|elem| elem.parse::<u64>())
                .flatten()
                .collect();

            operations.push((result, operands));
        }
        Ok(operations)
    }

    fn perform_operation(operation: Operation, left_operand: u64, right_operand: u64) -> u64 {
        match operation {
            Operation::Multiply => left_operand * right_operand,
            Operation::Add => left_operand + right_operand,
        }
    }

    fn create_permutations(len: usize) -> Vec<Vec<Operation>> {
        if len == 0 {
            return vec![vec![]];
        }

        let mut result = vec![];
        for perm in create_permutations(len - 1) {
            let mut multiply_perm = perm.clone();
            multiply_perm.push(Operation::Multiply);
            result.push(multiply_perm);

            let mut add_perm = perm.clone();
            add_perm.push(Operation::Add);
            result.push(add_perm);
        }
        result
    }

    fn check_operation(exp_result: u64, operands: &Vec<u64>) -> bool {
        let mut operands = operands.clone();
        operands.reverse();

        let permutations = create_permutations(operands.len() - 1);
        for operations in permutations {
            let mut current_operands = operands.clone();

            let mut res = current_operands.pop().unwrap();

            for operation in operations {
                let next_operand = current_operands.pop().unwrap();
                res = perform_operation(operation, res, next_operand);
            }

            if exp_result == res {
                return true;
            }
        }
        false
    }

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let operations = read_operations(reader)?;
        let count = operations
            .iter()
            .filter(|(result, operands)| check_operation(*result, operands))
            .map(|(result, _operands)| result)
            .sum::<u64>();

        Ok(count as usize)
    }

    assert_eq!(3749, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    #[derive(Clone, Hash, Eq, PartialEq, Debug)]
    enum Operation2 {
        Multiply,
        Add,
        Concatenate,
    }

    fn perform_operation2(operation: Operation2, left_operand: u64, right_operand: u64) -> u64 {
        match operation {
            Operation2::Multiply => left_operand * right_operand,
            Operation2::Add => left_operand + right_operand,
            Operation2::Concatenate => {
                format!("{}{}", left_operand.to_string(), right_operand.to_string())
                    .parse::<u64>()
                    .unwrap()
            }
        }
    }

    fn create_permutations2(len: usize) -> Vec<Vec<Operation2>> {
        if len == 0 {
            return vec![vec![]];
        }

        let mut result = vec![];
        for perm in create_permutations2(len - 1) {
            let mut multiply_perm = perm.clone();
            multiply_perm.push(Operation2::Multiply);
            result.push(multiply_perm);

            let mut add_perm = perm.clone();
            add_perm.push(Operation2::Add);
            result.push(add_perm);

            let mut concat_perm = perm.clone();
            concat_perm.push(Operation2::Concatenate);
            result.push(concat_perm);
        }
        result
    }

    fn check_operation2(exp_result: u64, operands: &Vec<u64>) -> bool {
        let mut operands = operands.clone();
        operands.reverse();

        let permutations = create_permutations2(operands.len() - 1);
        for operations in permutations {
            let mut current_operands = operands.clone();

            let mut res = current_operands.pop().unwrap();

            for operation in operations {
                let next_operand = current_operands.pop().unwrap();
                res = perform_operation2(operation, res, next_operand);
            }

            if exp_result == res {
                return true;
            }
        }
        false
    }

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let operations = read_operations(reader)?;
        let count = operations
            .iter()
            .filter(|(result, operands)| check_operation2(*result, operands))
            .map(|(result, _operands)| result)
            .sum::<u64>();

        Ok(count as usize)
    }

    assert_eq!(11387, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
