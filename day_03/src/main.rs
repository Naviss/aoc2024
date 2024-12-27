mod lexer;
mod parser;
use std::io::{self, BufRead};

use lexer::{tokenize, Token};
use parser::{parse, Expr};

fn solve2(lines: &Vec<String>) -> i64 {
    let mut somme = 0;
    let mut enable = true;
    let source = lines.join("\n");
    let tokens: Vec<Token> = tokenize(source.as_str()).collect();
    let res = parse(tokens).unwrap();
    for e in res.iter() {
        match e {
            Expr::Do { .. } => {
                enable = e.eval() == 1;
            }
            Expr::Mul { .. } => {
                if enable {
                    somme += e.eval();
                }
            }
            Expr::Lit { .. } => {
                continue;
            }
        }
    }
    somme
}

fn main() {
    let stdin = io::stdin();
    let mut sum1: i64 = 0;

    let lines = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    for line in &lines[..] {
        let tokens = tokenize(line.as_str()).collect();
        let res = parse(tokens).unwrap();
        sum1 += res.iter().map(|x| x.eval()).sum::<i64>();
    }

    let sum2 = solve2(&lines);

    println!("Day 03 - {} {}", sum1, sum2);
}

#[cfg(test)]
mod tests {
    use parser::Expr;

    use crate::lexer::tokenize;

    use super::*;
    #[test]
    fn test_solv2() {
        const TEST_STRING: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let tokens: Vec<Token> = tokenize(TEST_STRING).collect();
        let res = parse(tokens).unwrap();
        let mut somme = 0;
        let mut factor = 1;
        for e in res.iter() {
            match e {
                Expr::Do { .. } => factor = e.eval(),
                Expr::Mul { .. } => somme += e.eval() * factor,
                _ => continue,
            }
        }
        assert_eq!(somme, 48);
    }

    #[test]
    fn test_solv1() {
        const TEST_STRING: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let tokens: Vec<Token> = tokenize(TEST_STRING).collect();
        let res = parse(tokens).unwrap();
        let somme: i64 = res.iter().map(|x| x.eval()).sum();
        assert_eq!(somme, 161);
    }
}
