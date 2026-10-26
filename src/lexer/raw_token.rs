
/*  harper:ignore
    Spudbox. A library that implements lots of useful data types.
    Copyright (C) 2026 HappyPotato5 (on github)

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program. If not, see <https://www.gnu.org/licenses/>.
*/

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