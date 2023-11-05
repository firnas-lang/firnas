#![cfg(feature = "ar")]

use firnas_tokenizer::token::Literal;
use firnas_tokenizer::token::Token;
use firnas_tokenizer::token::TokenType;
use firnas_tokenizer::tokenizer::scan_tokens;

#[test]
fn it_should_tokenize_single_char_token() {
    assert_eq!(
        scan_tokens("( ) [ ] { } , . - + ; / *".to_owned()).unwrap(),
        vec![
            Token {
                ty: TokenType::LeftRoundBracket,
                lexeme: String::from("("),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightRoundBracket,
                lexeme: String::from(")"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LeftSquareBracket,
                lexeme: String::from("["),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightSquareBracket,
                lexeme: String::from("]"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LeftCurlyBracket,
                lexeme: String::from("{"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightCurlyBracket,
                lexeme: String::from("}"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Comma,
                lexeme: String::from(","),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Dot,
                lexeme: String::from("."),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Minus,
                lexeme: String::from("-"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Plus,
                lexeme: String::from("+"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Semicolon,
                lexeme: String::from(";"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Slash,
                lexeme: String::from("/"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Star,
                lexeme: String::from("*"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::new(),
                literal: None,
                line: 1,
                col: 0
            }
        ]
    );
}

#[test]
fn it_should_tokenize_one_or_two_char() {
    assert_eq!(
        scan_tokens("! != = == > >= < <=".to_owned()).unwrap(),
        vec![
            Token {
                ty: TokenType::Bang,
                lexeme: String::from("!"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::BangEqual,
                lexeme: String::from("!="),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Equal,
                lexeme: String::from("="),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::EqualEqual,
                lexeme: String::from("=="),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Greater,
                lexeme: String::from(">"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::GreaterEqual,
                lexeme: String::from(">="),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Less,
                lexeme: String::from("<"),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LessEqual,
                lexeme: String::from("<="),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from(""),
                literal: None,
                line: 1,
                col: 0
            },
        ]
    )
}

#[test]
fn it_should_tokenize_literals() {
    assert_eq!(
        scan_tokens("فرناس \"شئ\" ٣٫١٤".to_owned()).unwrap(),
        vec![
            Token {
                ty: TokenType::Identifier,
                lexeme: String::from("فرناس"),
                literal: Some(Literal::Identifier(String::from("فرناس"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::String,
                lexeme: String::from("\"شئ\""),
                literal: Some(Literal::Str(String::from("شئ"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Number,
                lexeme: String::from("٣٫١٤"),
                literal: Some(Literal::Number(3.14)),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from(""),
                literal: None,
                line: 1,
                col: 0
            }
        ]
    );
}

#[test]
fn it_should_tokenize_keywords() {
    assert_eq!(
        scan_tokens(
            "و صنف اخر خطا دالة من لو عدم او اطبع رد اساس هذا صحيح دع طالما لامدا".to_owned()
        )
        .unwrap(),
        vec![
            Token {
                ty: TokenType::And,
                lexeme: String::from("و"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Class,
                lexeme: String::from("صنف"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Else,
                lexeme: String::from("اخر"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::False,
                lexeme: String::from("خطا"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Fun,
                lexeme: String::from("دالة"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::For,
                lexeme: String::from("من"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::If,
                lexeme: String::from("لو"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Nil,
                lexeme: String::from("عدم"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Or,
                lexeme: String::from("او"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Print,
                lexeme: String::from("اطبع"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Return,
                lexeme: String::from("رد"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Super,
                lexeme: String::from("اساس"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::This,
                lexeme: String::from("هذا"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::True,
                lexeme: String::from("صحيح"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Var,
                lexeme: String::from("دع"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::While,
                lexeme: String::from("طالما"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Lambda,
                lexeme: String::from("لامدا"),
                literal: None,
                line: 1,
                col: 1,
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from(""),
                literal: None,
                line: 1,
                col: 1
            }
        ]
    );
}
