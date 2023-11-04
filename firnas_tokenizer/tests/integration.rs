use firnas_tokenizer::token::Literal;
use firnas_tokenizer::token::Token;
use firnas_tokenizer::token::TokenType;
use firnas_tokenizer::tokenizer;

#[test]
fn it_should_tokenize_single_char_token() {
    let _keywords_str =
        "and class else false fun for if nil or print return super this true var while lambda";

    assert_eq!(
        tokenizer::scan_tokens("( ) [ ] { } , . - + ; / *".to_owned()).unwrap(),
        vec![
            Token {
                ty: TokenType::LeftRoundBracket,
                lexeme: String::from("(").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightRoundBracket,
                lexeme: String::from(")").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LeftSquareBracket,
                lexeme: String::from("[").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightSquareBracket,
                lexeme: String::from("]").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LeftCurlyBracket,
                lexeme: String::from("{").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::RightCurlyBracket,
                lexeme: String::from("}").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Comma,
                lexeme: String::from(",").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Dot,
                lexeme: String::from(".").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Minus,
                lexeme: String::from("-").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Plus,
                lexeme: String::from("+").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Semicolon,
                lexeme: String::from(";").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Slash,
                lexeme: String::from("/").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Star,
                lexeme: String::from("*").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: vec![],
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
                lexeme: String::from("!").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::BangEqual,
                lexeme: String::from("!=").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Equal,
                lexeme: String::from("=").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::EqualEqual,
                lexeme: String::from("==").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Greater,
                lexeme: String::from(">").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::GreaterEqual,
                lexeme: String::from(">=").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Less,
                lexeme: String::from("<").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::LessEqual,
                lexeme: String::from("<=").into_bytes(),
                literal: None,
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from("").into_bytes(),
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
                lexeme: String::from("firnas").into_bytes(),
                literal: Some(Literal::Identifier(String::from("firnas"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::String,
                lexeme: String::from("\"str\"").into_bytes(),
                literal: Some(Literal::Str(String::from("str"))),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Number,
                lexeme: String::from("3.14").into_bytes(),
                literal: Some(Literal::Number(3.14)),
                line: 1,
                col: 0
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from("").into_bytes(),
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
                lexeme: String::from("and").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Class,
                lexeme: String::from("class").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Else,
                lexeme: String::from("else").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::False,
                lexeme: String::from("false").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Fun,
                lexeme: String::from("fun").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::For,
                lexeme: String::from("for").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::If,
                lexeme: String::from("if").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Nil,
                lexeme: String::from("nil").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Or,
                lexeme: String::from("or").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Print,
                lexeme: String::from("print").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Return,
                lexeme: String::from("return").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Super,
                lexeme: String::from("super").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::This,
                lexeme: String::from("this").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::True,
                lexeme: String::from("true").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Var,
                lexeme: String::from("var").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::While,
                lexeme: String::from("while").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Lambda,
                lexeme: String::from("lambda").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            },
            Token {
                ty: TokenType::Eof,
                lexeme: String::from("").into_bytes(),
                literal: None,
                line: 1,
                col: 1
            }
        ]
    );
}
