
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
pub use lexer::{Lexer, LexerError};