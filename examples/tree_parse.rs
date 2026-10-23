use spudbox::parse::*;

pub type BinaryTree = BTreeNode;

#[derive(Debug, PartialEq)]
pub enum BTreeNode {
    Leaf(&'static str),
    Branch{ left: Box<BTreeNode>, right: Box<BTreeNode> }
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftParen,
    RightParen,
    Lit(&'static str),
    Comma
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TokenType {
    LeftParen,
    RightParen,
    Lit,
    Comma
}

impl Kinded for Token {
    type Kind = TokenType;

    fn kind(&self) -> Self::Kind {
        match self {
            LeftParen => TokenType::LeftParen,
            RightParen => TokenType::RightParen,
            Lit(_) => TokenType::Lit,
            Comma => TokenType::Comma,
        }
    }
}

impl Parse<Token> for BinaryTree {
    fn parse(stream: ParserStream<Token>) -> Result<Self, ParseError> {
        let lit = stream.expect_token_kind(&TokenType::Lit);
        Ok(if let Ok(Token::Lit(lit)) = lit {
            BinaryTree::Leaf(*lit)
        } else {
            stream.expect_token(&Token::LeftParen)?;

            let l = Box::new(stream.parse()?);
            stream.expect_token(&Token::Comma)?;
            let r = Box::new(stream.parse()?);

            stream.expect_token(&Token::RightParen)?;

            BinaryTree::Branch { left: l, right: r }
        })
    }
}

use Token::*;

/// The same as `("a", (("b", "c"), "d"))`
const TOKENS: [Token; 13] = [LeftParen, Lit("a"), Comma, LeftParen, LeftParen, Lit("b"), Comma, Lit("c"), RightParen, Comma, Lit("d"), RightParen, RightParen];
const RESULT: &str = r#"Branch { left: Leaf("a"), right: Branch { left: Branch { left: Leaf("b"), right: Leaf("c") }, right: Leaf("d") } }"#;

pub fn main() {
    let buffer = ParserBuffer::new(&TOKENS);

    let tree = buffer.parse::<BinaryTree>().expect("Parse Error");

    assert_eq!(format!("{tree:?}"), RESULT);
}