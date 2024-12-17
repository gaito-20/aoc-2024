use anyhow::*;
use std::fs::File;
use std::io::{BufRead, BufReader};
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use adv_code_2024::*;

const DAY: &str = "17";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

trait Instruction {
    fn run(&self, computer: &mut Computer);
}

struct ADV {
    numerator: u32,
    denominator: u32
}

impl Instruction for ADV {
    fn run(&self, computer: &mut Computer) {
        computer.reg_a = self.numerator / self.denominator;
        computer.instruction_pointer += 2;
    }
}

struct BXL {
    left_operand: u32,
    right_operand: u32
}

impl Instruction for BXL {
    fn run(&self, computer: &mut Computer) {
        computer.reg_b = self.left_operand ^ self.right_operand;
        computer.instruction_pointer += 2;
    }
}

struct BST {
    operand: u32
}

impl Instruction for BST {
    fn run(&self, computer: &mut Computer) {
        computer.reg_b = self.operand % 8;
        computer.instruction_pointer += 2;
    }
}

struct JNZ {
    operand: u32
}

impl Instruction for JNZ {
    fn run(&self, computer: &mut Computer) {
        if computer.reg_a != 0 {
            computer.instruction_pointer = self.operand as usize;
        } else {
            computer.instruction_pointer += 2;
        }
    }
}

struct BXC {}

impl Instruction for BXC {
    fn run(&self, computer: &mut Computer) {
        computer.reg_b = computer.reg_b ^ computer.reg_c;
        computer.instruction_pointer += 2;
    }
}

struct OUT {
    operand: u32
}

impl Instruction for OUT {
    fn run(&self, computer: &mut Computer) {
        computer.output.push(self.operand % 8);
        computer.instruction_pointer += 2;
    }
}

struct BDV {
    numerator: u32,
    denominator: u32
}

impl Instruction for BDV {
    fn run(&self, computer: &mut Computer) {
        computer.reg_b = self.numerator / self.denominator;
        computer.instruction_pointer += 2;
    }
}

struct CDV {
    numerator: u32,
    denominator: u32
}

impl Instruction for CDV {
    fn run(&self, computer: &mut Computer) {
        computer.reg_c = self.numerator / self.denominator;
        computer.instruction_pointer += 2;
    }
}

struct Computer {
    reg_a: u32,
    reg_b: u32,
    reg_c: u32,
    program: Vec<u8>,
    instruction_pointer: usize,
    output: Vec<u32>,
}

impl Computer {
    fn from_input<R: BufRead>(reader: R) -> Self {
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let reg_a = lines.iter().find(|x| { x.contains("Register A") }).unwrap().strip_prefix("Register A: ").unwrap().parse::<u32>().unwrap();
        let reg_b = lines.iter().find(|x| { x.contains("Register B") }).unwrap().strip_prefix("Register B: ").unwrap().parse::<u32>().unwrap();
        let reg_c = lines.iter().find(|x| { x.contains("Register C") }).unwrap().strip_prefix("Register C: ").unwrap().parse::<u32>().unwrap();

        let program = lines.iter()
            .find(|x| { x.contains("Program") }).unwrap()
            .strip_prefix("Program: ").unwrap()
            .split(",")
            .map(|l| l.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();

        Self { reg_a, reg_b, reg_c, program, instruction_pointer: 0 , output: vec![] }
    }

    fn get_value_for_combo_operand(&self, combo_operand: u8) -> Result<u32> {
        match combo_operand {
            0_u8..=3_u8 => { Ok(combo_operand as u32) }
            4 => { Ok(self.reg_a) }
            5 => { Ok(self.reg_b) }
            6 => { Ok(self.reg_c) }
            _ => { Err(Error::msg("Invalid combo operand")) }
        }
    }

    fn next_instruction(&mut self) -> Result<Box<dyn Instruction>> {
        let opcode = self.program[self.instruction_pointer];
        let literal_operand = self.program[self.instruction_pointer + 1];
        let combo_operand = self.get_value_for_combo_operand(literal_operand).unwrap();

        match opcode {
            0 => Ok(Box::new(ADV { numerator: self.reg_a, denominator: 2_i32.pow(combo_operand) as u32 })),
            1 => Ok(Box::new(BXL { left_operand: self.reg_b, right_operand: literal_operand as u32 })),
            2 => Ok(Box::new(BST { operand: combo_operand })),
            3 => Ok(Box::new(JNZ { operand: literal_operand as u32 })),
            4 => Ok(Box::new(BXC { })),
            5 => Ok(Box::new(OUT { operand: combo_operand })),
            6 => Ok(Box::new(BDV { numerator: self.reg_a, denominator: 2_i32.pow(combo_operand) as u32 })),
            7 => Ok(Box::new(CDV { numerator: self.reg_a, denominator: 2_i32.pow(combo_operand) as u32 })),
            _ => Err(Error::msg("Invalid opcode"))
        }
    }

    fn run(&mut self) -> String {
        while self.instruction_pointer < self.program.len() {
            self.next_instruction().unwrap().run(self);
        }
        self.output.iter().join(",")
    }
}


fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<String> {
        let mut computer = Computer::from_input(reader);
        let output = computer.run();
        Ok(output)
    }

    assert_eq!("4,6,3,5,6,3,5,2,1,0", part1(BufReader::new(TEST.as_bytes()))?);

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
