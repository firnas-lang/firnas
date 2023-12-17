use crate::error::Error;
use crate::token::Literal;
use crate::token::Token;
use crate::token::TokenType;
use arabic_utils::arabic_char::ArabicChar;
use arabic_utils::arabic_str::ArabicStr;
use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

pub fn scan_tokens(input: String) -> Result<Vec<Token>, Error> {
    let mut scanner: Scanner = Default::default();

    scanner.scan_tokens(input);

    match scanner.err {
        Some(err) => Err(err),
        None => Ok(scanner.tokens),
    }
}

struct Scanner {
    source: Vec<String>,
    tokens: Vec<Token>,
    err: Option<Error>,
    start: usize,
    current: usize,
    line: usize,
    col: i64,
    keywords: HashMap<String, TokenType>,
}

impl Default for Scanner {
    fn default() -> Scanner {
        Scanner {
            source: Vec::new(),
            tokens: Vec::new(),
            err: None,
            start: 0,
            current: 0,
            line: 1,
            col: -1,
            keywords: vec![
                ("و", TokenType::And),
                ("صنف", TokenType::Class),
                ("غير_ذلك", TokenType::Else),
                ("خطا", TokenType::False),
                ("خطأ", TokenType::False),
                ("من", TokenType::For),
                ("دالة", TokenType::Fun),
                ("اذا_كان", TokenType::If),
                ("إذا_كان", TokenType::If),
                ("عدم", TokenType::Nil),
                ("او", TokenType::Or),
                ("أو", TokenType::Or),
                ("رد", TokenType::Return),
                ("اساس", TokenType::Super),
                ("أساس", TokenType::Super),
                ("هذا", TokenType::This),
                ("صح", TokenType::True),
                ("دع", TokenType::Var),
                ("طالما", TokenType::While),
                ("لامدا", TokenType::Lambda),
            ]
            .into_iter()
            .map(|(k, v)| (String::from(k), v))
            .collect(),
        }
    }
}

impl Scanner {
    fn scan_tokens(&mut self, input: String) {
        self.source = input
            .graphemes(true)
            .map(str::to_string)
            .collect::<Vec<_>>();

        while !self.done() {
            self.start = self.current;
            self.scan_token();
        }

        match self.err {
            Some(_) => {}
            None => self.tokens.push(Token {
                ty: TokenType::Eof,
                lexeme: String::new(),
                literal: None,
                line: self.line,
                col: self.col,
            }),
        }
    }

    fn advance(&mut self) -> String {
        self.current += 1;
        self.col += 1;

        self.source[self.current - 1].clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        let c = c.as_str();

        match c {
            "(" => self.add_token(TokenType::LeftRoundBracket),
            ")" => self.add_token(TokenType::RightRoundBracket),
            "{" => self.add_token(TokenType::LeftCurlyBracket),
            "}" => self.add_token(TokenType::RightCurlyBracket),
            "[" => self.add_token(TokenType::LeftSquareBracket),
            "]" => self.add_token(TokenType::RightSquareBracket),
            "," | "\u{060C}" => self.add_token(TokenType::Comma),
            "." => self.add_token(TokenType::Dot),
            "-" | "\u{2212}" => self.add_token(TokenType::Minus),
            "+" => self.add_token(TokenType::Plus),
            ";" | "\u{061B}" => self.add_token(TokenType::Semicolon),
            "*" => self.add_token(TokenType::Star),
            "!" => {
                let matches_eq = self.matches("=");
                self.add_token(if matches_eq {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                })
            }
            "=" => {
                let matches_eq = self.matches("=");
                self.add_token(if matches_eq {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                })
            }
            "<" => {
                let matches_eq = self.matches("=");
                self.add_token(if matches_eq {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                })
            }
            ">" => {
                let matches_eq = self.matches("=");
                self.add_token(if matches_eq {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                })
            }
            "\\" => {
                if self.matches("\\") {
                    while self.peek() != "\n" && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash)
                }
            }
            " " | "\r" | "\t" => {}
            "\n" => {
                self.line += 1;
                self.col = 0
            }
            "\"" => self.string(),
            _ => {
                if Scanner::is_decimal_digit(c) {
                    self.number()
                } else if Scanner::is_alpha(c) {
                    self.identifier()
                } else {
                    self.err = Some(Error {
                        what: format!("scanner can't handle {}", c),
                        line: self.line,
                        col: self.col,
                    })
                }
            }
        }
    }

    fn is_alpha(c: &str) -> bool {
        c.chars().all(char::is_arabic_alphabetic)
    }

    fn is_decimal_digit(c: &str) -> bool {
        c.is_arabic_number()
    }

    fn is_alphanumeric(c: &str) -> bool {
        Scanner::is_alpha(c) || Scanner::is_decimal_digit(c) || c == "_"
    }

    fn identifier(&mut self) {
        while Scanner::is_alphanumeric(&self.peek()) {
            self.advance();
        }

        let literal_val = self.source[self.start..self.current]
            .iter()
            .fold(String::new(), |cur, nxt| cur + nxt);

        let token_type = match self.keywords.get(&literal_val) {
            Some(kw_token_type) => *kw_token_type,
            None => TokenType::Identifier,
        };

        match token_type {
            TokenType::Identifier => self.add_token_literal(
                TokenType::Identifier,
                Some(Literal::Identifier(literal_val)),
            ),
            _ => self.add_token(token_type),
        }
    }

    fn number(&mut self) {
        while Scanner::is_decimal_digit(&self.peek()) {
            self.advance();
        }

        let decimal_sep = arabic_utils::arabic_consts::DECIMAL_SEPARATOR.to_string();

        if self.peek() == decimal_sep && Scanner::is_decimal_digit(&self.peek_next()) {
            self.advance();
        }

        while Scanner::is_decimal_digit(&self.peek()) {
            self.advance();
        }

        let val = self.source[self.start..self.current]
            .iter()
            .fold(String::new(), |cur, nxt| cur + nxt);

        let val: f64 = val.parse_arabic_decimal().unwrap().parse().unwrap();

        self.add_token_literal(TokenType::Number, Some(Literal::Number(val)))
    }

    fn string(&mut self) {
        while self.peek() != "\"" && !self.is_at_end() {
            if self.peek() == "\n" {
                self.line += 1
            }
            self.advance();
        }

        if self.is_at_end() {
            self.err = Some(Error {
                what: "Unterminated string".to_string(),
                line: self.line,
                col: self.col,
            })
        }

        assert!(self.peek() == "\"");

        self.advance();

        let value = self.source[self.start + 1..self.current - 1]
            .iter()
            .fold(String::new(), |cur, nxt| cur + nxt);

        self.add_token_literal(TokenType::String, Some(Literal::Str(value)))
    }

    fn peek_next(&self) -> String {
        if self.current + 1 >= self.source.len() {
            String::from("\0")
        } else {
            self.source[self.current + 1].clone()
        }
    }

    fn peek(&self) -> String {
        if self.is_at_end() {
            String::from("\0")
        } else {
            self.source[self.current].clone()
        }
    }

    fn matches(&mut self, c: &str) -> bool {
        if self.is_at_end() {
            return true;
        }

        if self.source[self.current] != c {
            return false;
        }

        self.current += 1;
        self.col += 1;
        true
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_literal(token_type, None)
    }

    fn add_token_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let lexeme = self.source[self.start..self.current]
            .iter()
            .fold(String::new(), |cur, nxt| cur + nxt);

        self.tokens.push(Token {
            ty: token_type,
            lexeme,
            literal,
            line: self.line,
            col: self.col,
        })
    }

    fn done(&self) -> bool {
        self.err.is_some() || self.is_at_end()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
