use std::{io, str::FromStr};

#[derive(Debug, PartialEq)]
struct Report {
    pub levels: Vec<i32>,
}

#[derive(Debug, PartialEq, Eq)]
struct ParseReportError;

impl FromStr for Report {
    type Err = ParseReportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let levels: Result<Vec<i32>, ParseReportError> = s
            .split_whitespace()
            .map(|str_level| str_level.parse::<i32>().map_err(|_| ParseReportError))
            .collect();
        Ok(Report { levels: levels? })
    }
}

fn is_good_report(levels: &Vec<i32>) -> bool {
    let dir = levels[1] - levels[0];
    let t = levels
        .windows(2)
        .map(|w| w[1] - w[0])
        .filter(|e| (e.abs() <= 3 && e.abs() > 0) && ((e ^ dir) >= 0))
        .count();
    t + 1 >= levels.len()
}

fn is_good_report_dumper(levels: &Vec<i32>) -> bool {
    for i in 0..levels.len() {
        let dampened: Vec<i32> = levels
            .iter()
            .enumerate()
            .filter(|&(j, _)| j != i)
            .map(|(_, e)| *e)
            .collect();
        if is_good_report(&dampened) {
            return true;
        }
    }
    false
}

fn solve_one(reports: &Vec<Report>) -> u32 {
    reports.iter().filter(|r| is_good_report(&r.levels)).count() as u32
}

fn solve_two(reports: &Vec<Report>) -> u32 {
    reports
        .iter()
        .filter(|r| is_good_report_dumper(&r.levels))
        .count() as u32
}

fn main() -> io::Result<()> {
    let lines = io::stdin().lines();
    let reports: Vec<Report> = lines
        .map(|l| Report::from_str(l.unwrap().as_str()).unwrap())
        .collect();
    let res1 = solve_one(&reports);
    let res2 = solve_two(&reports);
    println!("Day 02 - {} {}", res1, res2);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_parser() {
        let expected = Ok(Report {
            levels: vec![1, 2, 3, 45, 6, 7],
        });
        assert_eq!(Report::from_str("1 2 3 45 6  7"), expected);
        assert!(Report::from_str("we 3 4").is_err());
    }

    #[test]
    fn test_solve_one() {
        let reports = vec![
            Report::from_str("7 6 4 2 1").unwrap(),
            Report::from_str("1 2 7 8 9").unwrap(),
            Report::from_str("9 7 6 2 1").unwrap(),
            Report::from_str("1 3 2 4 5").unwrap(),
            Report::from_str("8 6 4 4 1").unwrap(),
            Report::from_str("1 3 6 7 9").unwrap(),
        ];
        let res = solve_one(&reports);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_solve_two() {
        let reports = vec![
            Report::from_str("7 6 4 2 1").unwrap(),
            Report::from_str("1 2 7 8 9").unwrap(),
            Report::from_str("9 7 6 2 1").unwrap(),
            Report::from_str("1 3 2 4 5").unwrap(),
            Report::from_str("8 6 4 4 1").unwrap(),
            Report::from_str("1 3 6 7 9").unwrap(),
        ];
        let res = solve_two(&reports);
        assert_eq!(res, 4);
    }
}
