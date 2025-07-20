use crate::Solution;

pub struct Day02;
impl Solution for Day02 {
    fn challenge1(&self, input: &str) -> String {
        input.lines()
            .map(|line| Report::from_line(line))
            .filter(|report| match report.check_report() {
                ReportStatus::Safe(_) => true,
                ReportStatus::Unsafe => false,
            })
            .count()
            .to_string()
    }

    fn challenge2(&self, input: &str) -> String {
        input.lines()
            .map(|line| Report::from_line(line))
            .filter(|report| match report.tolerant_report_check() {
                ReportStatus::Safe(_) => true,
                ReportStatus::Unsafe => false,
            })
            .count()
            .to_string()
    }
}

struct Report {
    levels: Vec<u32>
}

enum ReportDirection {
    INCREASING,
    DECREASING
}

enum ReportStatus {
    Safe(ReportDirection),
    Unsafe
}

impl Report {
    fn from_line(line: &str) -> Self {
        let levels: Vec<u32> = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        Self {
            levels
        }
    }

    fn check_report(&self) -> ReportStatus {
        let levels_iter = self.levels.iter();
        let increasing = levels_iter.clone().is_sorted();
        let decreasing = levels_iter.clone().rev().is_sorted();
        let level_diff_valid = self.levels.windows(2)
            .all(|window| {
                let diff = window[1].abs_diff(window[0]);
                diff >= 1 && diff <=3
        });


        if level_diff_valid {
            if increasing {
                return ReportStatus::Safe(ReportDirection::INCREASING);
            }
            if decreasing {
                return ReportStatus::Safe(ReportDirection::DECREASING);
            }
        }
        ReportStatus::Unsafe
    }

    fn tolerant_report_check(&self) -> ReportStatus {
        if let ReportStatus::Safe(direction)= self.check_report() {
            return ReportStatus::Safe(direction);
        }

        for i in 0..self.levels.len() {
            let mut levels = self.levels.clone();
            levels.remove(i);
            let mod_report = Report { levels };
            if let ReportStatus::Safe(direction) = mod_report.check_report() {
                return ReportStatus::Safe(direction);
            }
        }
        ReportStatus::Unsafe
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    #[test]
    fn test_challenge1() {
        let day = Day02 {};
        assert_eq!(day.challenge1(INPUT), "2");
    }

    #[test]
    fn test_challenge2() {
        let day = Day02 {};
        assert_eq!(day.challenge2(INPUT), "4");
    }
}