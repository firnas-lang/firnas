use firnas_tokenizer::token::Literal;
use firnas_tokenizer::token::Token;
use firnas_tokenizer::token::TokenType;
use firnas_tokenizer::tokenizer;

#[test]
fn it_should_tokenize_single_char_token() {
    assert_eq!(
        tokenizer::scan_tokens("( ) [ ] { } , . - + ; / *".to_owned()).unwrap(),
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
        tokenizer::scan_tokens("! != = == > >= < <=".to_owned()).unwrap(),
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
        tokenizer::scan_tokens("firnas \"str\" 3.14".to_owned()).unwrap(),
        vec![
            Token {
                ty: TokenType::Identifier,
                lexeme: String::from("firnas"),
                literal: Some(Literal::Identifier(String::from("firnas"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::String,
                lexeme: String::from("\"str\""),
                literal: Some(Literal::Str(String::from("str"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Number,
                lexeme: String::from("3.14"),
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
        tokenizer::scan_tokens(
            "and class else false fun for if nil or print return super this true var while lambda"
                .to_owned()
        )
        .unwrap(),
        vec![
            Token {
                ty: TokenType::And,
                lexeme: String::from("and"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Class,
                lexeme: String::from("class"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Else,
                lexeme: String::from("else"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::False,
                lexeme: String::from("false"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Fun,
                lexeme: String::from("fun"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::For,
                lexeme: String::from("for"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::If,
                lexeme: String::from("if"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Nil,
                lexeme: String::from("nil"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Or,
                lexeme: String::from("or"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Print,
                lexeme: String::from("print"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Return,
                lexeme: String::from("return"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Super,
                lexeme: String::from("super"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::This,
                lexeme: String::from("this"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::True,
                lexeme: String::from("true"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Var,
                lexeme: String::from("var"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::While,
                lexeme: String::from("while"),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Lambda,
                lexeme: String::from("lambda"),
                literal: None,
                line: 1,
                col: 1
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
