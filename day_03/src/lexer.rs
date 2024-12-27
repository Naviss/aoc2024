#[repr(u16)]
#[derive(Debug, PartialEq, Eq)]
pub enum SyntaxKind {
    Mul,
    Integer,
    LeftParenthesis,
    RightParenthesis,
    Comma,
    Glitch,
    Do,
    Dont,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Token<'a> {
    pub kind: SyntaxKind,
    pub len: usize,
    pub value: Option<&'a str>,
}

pub fn tokenize(source: &str) -> impl Iterator<Item = Token> + '_ {
    let mut s = source;

    std::iter::from_fn(move || {
        if s.is_empty() {
            return None;
        }
        let token = lex(s);
        s = &s[token.len..];
        Some(token)
    })
}

pub fn lex(source: &str) -> Token {
    if source.starts_with("mul") {
        Token {
            kind: SyntaxKind::Mul,
            len: 3,
            value: Some(&source[..3]),
        }
    } else if source.starts_with("(") {
        Token {
            kind: SyntaxKind::LeftParenthesis,
            len: 1,
            value: Some(&source[..1]),
        }
    } else if source.starts_with(")") {
        Token {
            kind: SyntaxKind::RightParenthesis,
            len: 1,
            value: Some(&source[..1]),
        }
    } else if source.starts_with(",") {
        Token {
            kind: SyntaxKind::Comma,
            len: 1,
            value: Some(&source[..1]),
        }
    } else if source.starts_with(is_digit) {
        let len = source.find(is_not_digit).unwrap_or(source.len());
        if len > 3 {
            Token {
                kind: SyntaxKind::Glitch,
                len,
                value: Some(&source[..len]),
            }
        } else {
            Token {
                kind: SyntaxKind::Integer,
                len,
                value: Some(&source[..len]),
            }
        }
    } else if source.starts_with("don't()") {
        Token {
            kind: SyntaxKind::Dont,
            len: 7,
            value: Some(&source[..7]),
        }
    } else if source.starts_with("do()") {
        Token {
            kind: SyntaxKind::Do,
            len: 4,
            value: Some(&source[..4]),
        }
    } else {
        Token {
            kind: SyntaxKind::Glitch,
            len: 1,
            value: Some(&source[..1]),
        }
    }
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn is_not_digit(c: char) -> bool {
    !is_digit(c)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_do() {
        let source: &str = &"do()xmul(2,4)don't()";
        let tokens: Vec<Token> = tokenize(source).collect();
        let expected = vec![
            Token {
                kind: SyntaxKind::Do,
                len: 4,
                value: Some(&source[0..4]),
            },
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[4..5]),
            },
            Token {
                kind: SyntaxKind::Mul,
                len: 3,
                value: Some(&source[5..8]),
            },
            Token {
                kind: SyntaxKind::LeftParenthesis,
                len: 1,
                value: Some(&source[8..9]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[9..10]),
            },
            Token {
                kind: SyntaxKind::Comma,
                len: 1,
                value: Some(&source[10..11]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[11..12]),
            },
            Token {
                kind: SyntaxKind::RightParenthesis,
                len: 1,
                value: Some(&source[12..13]),
            },
            Token {
                kind: SyntaxKind::Dont,
                len: 7,
                value: Some(&source[13..20]),
            },
        ];
        assert_eq!(tokens.len(), expected.len());
        for t in std::iter::zip(tokens, expected) {
            assert_eq!(t.0, t.1);
        }
    }

    #[test]
    fn test_lexer() {
        let source: &str = &"xmul(2,4)%&mul[3,7]";
        let tokens: Vec<Token> = tokenize(source).collect();
        let expected = vec![
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[0..1]),
            },
            Token {
                kind: SyntaxKind::Mul,
                len: 3,
                value: Some(&source[1..4]),
            },
            Token {
                kind: SyntaxKind::LeftParenthesis,
                len: 1,
                value: Some(&source[4..5]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[5..6]),
            },
            Token {
                kind: SyntaxKind::Comma,
                len: 1,
                value: Some(&source[6..7]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[7..8]),
            },
            Token {
                kind: SyntaxKind::RightParenthesis,
                len: 1,
                value: Some(&source[8..9]),
            },
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[9..10]),
            },
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[10..11]),
            },
            Token {
                kind: SyntaxKind::Mul,
                len: 3,
                value: Some(&source[11..14]),
            },
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[14..15]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[15..16]),
            },
            Token {
                kind: SyntaxKind::Comma,
                len: 1,
                value: Some(&source[16..17]),
            },
            Token {
                kind: SyntaxKind::Integer,
                len: 1,
                value: Some(&source[17..18]),
            },
            Token {
                kind: SyntaxKind::Glitch,
                len: 1,
                value: Some(&source[18..19]),
            },
        ];
        assert_eq!(tokens.len(), expected.len());
        for t in std::iter::zip(tokens, expected) {
            assert_eq!(t.0, t.1);
        }
    }
}
