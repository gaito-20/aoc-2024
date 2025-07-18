pub mod days;
pub use days::day01::Day01;

pub struct AdventOfCode {
    pub days: Vec<Day>,
}

pub struct Day {
    pub day: u32,
    pub challenge_string: Option<String>,
    pub solution: Option<Box<dyn Solution>>,
}

pub trait Solution {
    fn challenge1(&self, input: &str) -> String;
    fn challenge2(&self, input: &str) -> String;
}

impl Day {
    pub fn solve(&self) -> (String, String) {
        match self.solution {
            None => ("Not solved yet".to_string(), "Not solved yet".to_string()),
            Some(ref solution) => (
                solution.challenge1(self.challenge_string.as_ref().unwrap()),
                solution.challenge2(self.challenge_string.as_ref().unwrap()),
            ),
        }
    }
}

impl AdventOfCode {
    pub fn new() -> Self {
        let days: Vec<Day> = (1..=24)
            .map(|day| {
                let challenge_string =
                    std::fs::read_to_string(format!("input/{:0>2}.txt", day)).ok();
                Day {
                    day,
                    challenge_string,
                    solution: None,
                }
            })
            .collect();
        AdventOfCode { days }
    }

    pub fn add_solution(&mut self, day: u32, solution: Box<dyn Solution>) {
        if let Some(d) = self.days.iter_mut().find(|d| d.day == day) {
            d.solution = Some(solution);
        } else {
            eprintln!("Day {} not found!", day);
        }
    }
}

pub fn start_day(day: &str) {
    println!("Advent of Code 2024 - Day {:0>2}", day);
}

// Additional common functions

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        start_day("00");
    }
}
