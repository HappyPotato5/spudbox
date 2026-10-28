
#![allow(unused)]

use spudbox::parse::*;

// harper:ignore
// <!--vale off-->
// Grammar (in EBNF):
// Add := Mul "+" Add | Mul;
// Mul := Num "*" Mul | Num;
// Num := u32 | "(" Add ")";
// <!--vale on-->

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    OpenParen,
    CloseParen,
    Add,
    Mul,
    Int(u32)
}

pub type AST = Add;

#[derive(Debug)]
pub struct Add {
    a: Mul,
    b: Option<Box<Add>>
}

#[derive(Debug)]
pub struct Mul {
    a: Num,
    b: Option<Box<Mul>>
}

#[derive(Debug)]
pub enum Num {
    Group(Box<Add>),
    Literal(u32)
}

impl Parse<Token> for Add {
    type Err = ParseError;

    fn parse(stream: ParserStream<Token>) -> Result<Self, ParseError> {
        let a = stream.parse()?;

        let mut b = None;
        if stream.expect_token(&Token::Add).is_ok() {
            b = Some(Box::new(stream.parse()?));
        }

        Ok(Add {a, b})
    }
}

impl Parse<Token> for Mul {
    type Err = ParseError;

    fn parse(stream: ParserStream<Token>) -> Result<Self, ParseError> {
        let a = stream.parse()?;

        let mut b = None;
        if stream.expect_token(&Token::Mul).is_ok() {
            b = Some(Box::new(stream.parse()?));
        }

        Ok(Mul {a, b})
    }
}

impl Parse<Token> for Num {
    type Err = ParseError;

    fn parse(stream: ParserStream<Token>) -> Result<Self, ParseError> {
        if stream.expect_token(&Token::OpenParen).is_ok() {
            let add = stream.parse()?;
            stream.expect_token_def(&Token::CloseParen)?;

            return Ok(Num::Group(Box::new(add)))
        }

        let mut num = 0;
        stream.expect_def(|x| match x { Token::Int(i) => {num = *i; true}, _ => false })?;

        Ok(Num::Literal(num))
    }
}

// Same as `11 + 1 * (2 + 55 * 4 + 123)` after *lexing*.
const TOKEN_STREAM: [Token; 13] = [Token::Int(11), Token::Add, Token::Int(1), Token::Mul, Token::OpenParen, Token::Int(2), Token::Add, Token::Int(55), Token::Mul, Token::Int(4), Token::Add, Token::Int(123), Token::CloseParen];
const RESULT: &str = r#"Add { a: Mul { a: Literal(11), b: None }, b: Some(Add { a: Mul { a: Literal(1), b: Some(Mul { a: Group(Add { a: Mul { a: Literal(2), b: None }, b: Some(Add { a: Mul { a: Literal(55), b: Some(Mul { a: Literal(4), b: None }) }, b: Some(Add { a: Mul { a: Literal(123), b: None }, b: None }) }) }), b: None }) }, b: None }) }"#;

#[test]
fn parse_test() {
    let stream = ParserBuffer::new(&TOKEN_STREAM);

    let add = stream.parse::<Add>().expect("Error during parsing");

    assert_eq!(format!("{:?}", add), RESULT);
}