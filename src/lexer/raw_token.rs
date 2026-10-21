
use super::Span;

/// A [`RawToken`] contains a `TokenKind`, a `lexeme`, and a [`Span`].
/// 
/// The **Payload** of a token carries **additional** data that it may contain. For example: an `Int` token can contain a [`u32`].
/// 
/// # Generics
/// `TokenKind` represents an **enum** of all the possible *token kinds*, **without payload**. (only *unit variants*)
/// 
/// # Fields
/// + `kind` stores the kind of the resulting token.
/// + `lexeme` stores the **lexeme** this token represents.
/// + [`span`](Span) stores the starting and ending position of the token (in bytes).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawToken<TokenKind> {
    pub kind: TokenKind,
    pub lexeme: String,
    pub span: Span,
}

impl<TokenKind> RawToken<TokenKind> {
    /// Construct a new [`RawToken`] with the given `kind`, `lexeme` and `span`.
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> RawToken<TokenKind>{
        RawToken { kind, lexeme, span }
    }
}