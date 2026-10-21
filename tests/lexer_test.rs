
use spudbox::lexer::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    LeftParen,
    RightParen,
    Add,
    AddAssign,
    Sub,
    SubAssign,
    Mul,
    MulAssign,
    Semicolon,

    Int,
    Ident,
    Space,
}

struct AnyInt;

impl Pattern for AnyInt {
    fn matches(&self, remaining: &str) -> usize {
        let mut len = 0;
        for (i, c) in remaining.char_indices() {
            if c.is_digit(10) {
                len = i + c.len_utf8();
            } else { break }
        }
        len
    }
}

struct AnyIdent;

impl Pattern for AnyIdent {
    fn matches(&self, remaining: &str) -> usize {
        let mut len = 0;
        for (i, c) in remaining.char_indices() {
            if c.is_alphabetic() {
                len = i + c.len_utf8();
            } else { break }
        }
        len
    }
}

struct AnySpace;

impl Pattern for AnySpace {
    fn matches(&self, remaining: &str) -> usize {
        let mut len = 0;
        for (i, c) in remaining.char_indices() {
            if c.is_whitespace() {
                len = i + c.len_utf8();
            } else { break }
        }
        len
    }
}

const INPUT_STR: &str = "hola += 10; 1 + 2 * 3;";
const EXPECTED: &str = r#"[RawToken { kind: Ident, lexeme: "hola", span: Span { start: 0, end: 3 } }, RawToken { kind: AddAssign, lexeme: "+=", span: Span { start: 5, end: 6 } }, RawToken { kind: Int, lexeme: "10", span: Span { start: 8, end: 9 } }, RawToken { kind: Semicolon, lexeme: ";", span: Span { start: 10, end: 10 } }, RawToken { kind: Int, lexeme: "1", span: Span { start: 12, end: 12 } }, RawToken { kind: Add, lexeme: "+", span: Span { start: 14, end: 14 } }, RawToken { kind: Int, lexeme: "2", span: Span { start: 16, end: 16 } }, RawToken { kind: Mul, lexeme: "*", span: Span { start: 18, end: 18 } }, RawToken { kind: Int, lexeme: "3", span: Span { start: 20, end: 20 } }, RawToken { kind: Semicolon, lexeme: ";", span: Span { start: 21, end: 21 } }]"#;

#[test]
fn lexer_test() {
    use TokenKind::*;

    let recipes = recipe_list![
        "(", LeftParen;
        ")", RightParen;
        "+", Add;
        "+=", AddAssign;
        "-", Sub;
        "-=", SubAssign;
        "*", Mul;
        "*=", MulAssign;
        ";", Semicolon;
        AnyInt, Int;
        AnyIdent, Ident;
        AnySpace, Space;
    ];

    println!("{}", Pattern::matches("+=", "+=1234"));

    let mut lexer = Lexer::with_recipe_list(ConflictPolicy::Greedy, recipes);
    lexer.ignore(Space);

    let tokens = lexer.tokenize(INPUT_STR).expect("Error during tokenization");

    assert_eq!(format!("{tokens:?}"), EXPECTED)
}