
use spudbox::lexer::*;

// All the possible token kinds.
//
// Note that *Int doesn't carry an int*. You can *construct it later with the token's lexeme*.
//
// Only unit variants.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TokenKind {
    LeftParen,
    RightParen,
    Add,
    Sub,
    Mul,
    Div,

    Int,
    Space,
}

struct AnyInt;

// This matches any sequence of decimal digits.
impl Pattern for AnyInt {
    fn matches(&self, remaining: &str) -> usize {
        let mut len = 0;
        for (i, c) in remaining.char_indices() {
            if c.is_digit(10) { len = i + c.len_utf8() } else { break }
        }
        len
    }
}

struct AnySpace;

// This matches any sequence of spaces.
impl Pattern for AnySpace {
    fn matches(&self, remaining: &str) -> usize {
        let mut len = 0;
        for (i, c) in remaining.char_indices() {
            if c.is_whitespace() { len = i + c.len_utf8() } else { break }
        }
        len
    }
}

pub fn main() {
    use TokenKind::*;

    // First, create the recipe list.
    let recipes = recipe_list![
        "(", LeftParen;
        ")", RightParen;
        "+", Add;
        "-", Sub;
        "*", Mul;
        "/", Div;

        AnyInt, Int;
        AnySpace, Space;
    ];

    // Then the lexer
    let mut lexer = Lexer::with_recipe_list(ConflictPolicy::Greedy, recipes);

    // Ignore spaces
    lexer.ignore(Space);

    // Tokenize
    let tokens = lexer.tokenize("1 + 2 * (11 - 233) / 12345 + 12345").expect("Lexer error");
    
    // Ignore, just for the assert below.
    let mut ints = Vec::new();
    let kinds: Vec<_> = tokens.into_iter().map(
        |tok| {
            if tok.kind == Int {
                ints.push(tok.lexeme);
            }
            tok.kind
        }
    ).collect();
    
    assert_eq!(format!("{kinds:?}"), "[Int, Add, Int, Mul, LeftParen, Int, Sub, Int, RightParen, Div, Int, Add, Int]");
    assert_eq!(format!("{ints:?}"), r#"["1", "2", "11", "233", "12345", "12345"]"#);
}