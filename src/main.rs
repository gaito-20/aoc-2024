use adv_code_2024::{AdventOfCode, Day01, Day02};


fn main() {
    let mut aoc = AdventOfCode::new();
    aoc.add_solution(01, Box::new(Day01 {}));
    aoc.add_solution(02, Box::new(Day02 {}));

    println!("=== Advent of Code 2024 ===");
    for day in aoc.days {
        let (solution1, solution2) = day.solve();
        println!(
            "Day {:0>2} - Part 1: {}, Part 2: {}",
            day.day, solution1, solution2
        );
    }
}
