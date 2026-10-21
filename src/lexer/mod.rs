//! A submodule which adds tools for lexing tokens from strings, like: [`Lexer`], [`Pattern`], [`RawToken`], [`Span`].

mod span;
mod raw_token;
mod pattern;
mod recipe;
mod lexer;

pub use span::Span;
pub use raw_token::RawToken;
pub use pattern::{Pattern, ConflictPolicy};
pub use recipe::{Recipe, recipe, recipe_list};
pub use lexer::Lexer;