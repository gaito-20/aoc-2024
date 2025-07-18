pub mod days;

pub struct AdventOfCode {
    pub days: Vec<Day>,
}

pub struct Day {
    pub day: u32,
    pub solution: Option<Box<dyn Solution>>
}

pub trait Solution {
    fn challenge1(&self) -> String;
    fn challenge2(&self) -> String;
}


impl Day {
    pub fn solve(&self) -> (String, String) {
        match self.solution {
            None => ("Not solved yet".to_string(), "Not solved yet".to_string()),
            Some(ref solution) => (solution.challenge1(), solution.challenge2()),
        }
    }
}

impl AdventOfCode {
    pub fn new() -> Self {
        let days: Vec<Day> = (1..=24).map(|day| Day { day, solution: None } ).collect();
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
