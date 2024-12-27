use crate::{lexer::SyntaxKind, Token};
use thiserror::Error;

#[derive(Error, Debug)]
enum ParserError {
    #[error("Syntax error in multiplication parsing")]
    ParseMulSystax,

    #[error("Value error in token passed to multiplication parser")]
    ParseMulEmptyValue,

    #[error("A or B value Error")]
    AorB,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Expr {
    Mul { a: Box<Expr>, b: Box<Expr> },
    Lit { value: i64 },
    Do { value: i64 },
}

impl Expr {
    pub fn eval(&self) -> i64 {
        match &self {
            Expr::Mul { a, b } => a.eval() * b.eval(),
            Expr::Lit { value } => *value,
            Expr::Do { value } => *value,
        }
    }
}

pub fn parse_mul(tokens: &[Token]) -> Result<Expr, Box<dyn std::error::Error>> {
    let mut a = None;
    let mut b = None;

    if tokens[0].kind != SyntaxKind::LeftParenthesis
        || tokens[2].kind != SyntaxKind::Comma
        || tokens[4].kind != SyntaxKind::RightParenthesis
    {
        return Err(Box::new(ParserError::ParseMulSystax));
    } else {
        if tokens[1].kind == SyntaxKind::Integer {
            if let Some(val) = tokens[1].value {
                let number = val.parse::<i64>()?;
                a = Some(Expr::Lit { value: number });
            } else {
                return Err(Box::new(ParserError::ParseMulEmptyValue));
            }
        }
        if tokens[3].kind == SyntaxKind::Integer {
            if let Some(val) = tokens[3].value {
                let number = val.parse::<i64>()?;
                b = Some(Expr::Lit { value: number });
            } else {
                return Err(Box::new(ParserError::ParseMulEmptyValue));
            }
        }
    }
    if a.is_none() || b.is_none() {
        return Err(Box::new(ParserError::AorB));
    } else {
        return Ok(Expr::Mul {
            a: Box::new(a.unwrap()),
            b: Box::new(b.unwrap()),
        });
    }
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expr>, Box<dyn std::error::Error>> {
    let mut token_it = tokens.iter();
    let mut exprs: Vec<Expr> = vec![];
    while let Some(t) = token_it.next() {
        match t {
            Token {
                kind: SyntaxKind::Glitch,
                ..
            } => {
                continue;
            }
            Token {
                kind: SyntaxKind::Mul,
                ..
            } => {
                let ex = parse_mul(token_it.as_slice());
                if ex.is_ok() {
                    exprs.push(ex.unwrap());
                    token_it.nth(4);
                }
            }
            Token {
                kind: SyntaxKind::Do,
                ..
            } => exprs.push(Expr::Do { value: 1 }),
            Token {
                kind: SyntaxKind::Dont,
                ..
            } => exprs.push(Expr::Do { value: 0 }),
            _ => {
                continue;
            }
        }
    }
    Ok(exprs)
}

#[cfg(test)]
mod tests {
    use crate::lexer::tokenize;

    use super::*;

    #[test]
    fn test_parse() {
        let source: &str = &"xmul(2,4)%&mul[3,7]";
        let tokens = tokenize(source).collect();
        let res = parse(tokens).unwrap();
        let expected = vec![Expr::Mul {
            a: Box::new(Expr::Lit { value: 2 }),
            b: Box::new(Expr::Lit { value: 4 }),
        }];

        let sum = res.iter().map(|x| x.eval()).sum::<i64>();
        assert_eq!(res, expected);
        assert_eq!(sum, 8);
    }
}
