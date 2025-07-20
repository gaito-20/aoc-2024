use itertools::Itertools;

use crate::Solution;

pub struct Day01 {}
impl Solution for Day01 {
    fn challenge1(&self, input: &str) -> String {
        let (mut left, mut right) = parse_challenge(input);
        let mut distance = 0;

        while !left.is_empty() {
            let left_min = pop_minimum(&mut left);
            let right_min = pop_minimum(&mut right);

            distance += (left_min - right_min).abs();
        }

        distance.to_string()
    }

    fn challenge2(&self, input: &str) -> String {
        let (left, right) = parse_challenge(input);

        let right_map = right.iter().counts();
        let mut distance = 0;

        for elem in left {
            distance += (elem as usize) * right_map.get(&elem).unwrap_or(&0);
        }

        distance.to_string()
    }
}

fn parse_challenge(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut iter = line.split_whitespace();
        if let (Some(l), Some(r)) = (iter.next(), iter.next()) {
            left.push(l.parse::<i32>().unwrap());
            right.push(r.parse::<i32>().unwrap());
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "\
3   4
4   3
2   5
1   3
3   9
3   3
";

    #[test]
    fn test_challenge1() {
        let day: Day01 = Day01 {};
        assert_eq!(day.challenge1(INPUT), "11");
    }

    #[test]
    fn test_challenge2() {
        let day = Day01 {};
        assert_eq!(day.challenge2(INPUT), "31");
    }
}
