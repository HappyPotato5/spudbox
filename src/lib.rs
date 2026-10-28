
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

#![allow(rustdoc::redundant_explicit_links)]

//! [![Crates.io](https://img.shields.io/badge/Crates.io-latest-blue?style=for-the-badge&logo=rust&logoColor=black&labelColor=BBBBBB)](https://crates.io/crates/spudbox)
//! [![Docs.rs](https://img.shields.io/badge/Documentation-SpudBox-blue?style=for-the-badge&logo=Docs.rs&logoColor=black&labelColor=BBBBBB)](https://docs.rs/spudbox/latest/spudbox/)
//! [![GitHub](https://img.shields.io/badge/Github-SpudBox-blue?style=for-the-badge&logo=github&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox)
//! [![License](https://img.shields.io/badge/License-GPL--v3-blue?style=for-the-badge&logo=readdotcv&logoColor=black&labelColor=BBBBBB)](https://github.com/HappyPotato5/spudbox/blob/master/LICENSE)
//! 
//! # Spudbox
//! A library that implements lots of useful **data structures** used in <!--vale off--> my <!--vale on--> **Rust** projects.
//! 
//! ## Types
//! 
//! From **[`prelude`](mod@prelude)**:
//! + [`VecSet<T>`](vecset::VecSet): A [`Vec<T>`](std::vec::Vec) that doesn't keep repetitions.
//! + [`OrderedSet<T>`](ordered_set::OrderedSet): An **ordered** [`HashSet<T>`](std::collections::HashSet).
//! + [`Arena<T>`](arena::Arena): An **Arena allocator** for any **T** type.
//! 
//! From **[`lexer`](mod@lexer)**:
//! + [`Lexer`](lexer::Lexer): A **lexer** for tokens matching a [`Pattern`](lexer::Pattern).
//! + [`RawToken`](lexer::RawToken): A **Token** which still contains the *lexeme*. 
//! + [`Span`](lexer::Span): A struct that holds the starting and end position of a [`RawToken`](lexer::RawToken).
//! + [`Pattern`](lexer::Pattern): A **pattern** that can match a *substring*.
//! + [`ConflictPolicy`](lexer::ConflictPolicy): An *enum* used to resolve conflicts.
//! + [`Recipe`](lexer::Recipe): A **recipe** used by a [`Lexer`](lexer::Lexer) to build *tokens*.
//! + [`recipe!`](recipe): A **macro** that constructs [`Recipes`](lexer::Recipe) in a simplified manner.
//! + [`recipe_list!`](recipe_list): A **macro** that constructs [`Vec<Recipe>`](lexer::Recipe) in a simplified manner.
//! 
//! From **[`parse`](mod@parse)**:
//! + [`ParserStream`](parse::ParserStream): A **stream** able to [parse](parse::Parse) items from *tokens*, in a *top-down manner*.
//! + [`ParserBuffer`](parse::ParserBuffer): The buffer that [`ParserStream`](parse::ParserStream) uses.
//! + [`Parse`](parse::Parse): A **trait** implemented by any type that a [`ParserStream`](parse::ParserStream) can construct from a *sequence of tokens*.
//! + [`Kinded`](parse::Kinded): A **trait** implemented by a *token type*, providing a getter to its *kind*.
//! + [`ParseError`](parse::ParseError): The *default* **error type** returned when the [`ParserStream`](parse::ParserStream) couldn't parse an *item*.
//! 
//! ## Features
//! This crate uses **features**, so you only import the submodules that **you use**.
//! 
//! ### <!--vale off-->By Default<!--vale on-->:
//! + **`vecset`**: Enables the [`vecset`](mod@vecset) submodule.
//! + **`ordered-set`**: Enables the [`ordered_set`](mod@ordered_set) submodule.
//! + **`arena`**: Enables the [`arena`](mod@arena) submodule.
//! 
//! ### Additionally:
//! + **`lexer`**: Enables the [`lexer`](mod@lexer) submodule.
//! + **`parse`**: Enables the [`parse`](mod@parse) submodule.
//! 
//! **Spudbox** crate automatically **re-exports** prelude contents, to avoid this you can use the **`no-prelude`** **feature**.
//! 
//! ## Examples
//! You can find examples in the [GitHub repository](https://github.com/HappyPotato5/spudbox).
//! 

#[cfg(feature = "arena")]
pub mod arena;

#[cfg(feature = "ordered-set")]
pub mod ordered_set;

#[cfg(feature = "vecset")]
pub mod vecset;

#[cfg(feature = "parse")]
pub mod parse;

#[cfg(feature = "lexer")]
pub mod lexer;

/// The submodule that contains the most used types.
/// 
/// Prelude contains the following types (if enabled):
/// + [`arena::Arena`](mod@arena)
/// + [`ordered_set::*`](mod@ordered_set)
/// + [`vecset::*`](mod@vecset)`
pub mod prelude {
    #![allow(unused)]
    use super::*;

    #[cfg(any(feature = "arena"))]
    pub use arena::*;

    #[cfg(any(feature = "ordered-set"))]
    pub use ordered_set::*;

    #[cfg(any(feature = "vecset"))]
    pub use vecset::*;
}

#[allow(unused)]
#[cfg(not(feature = "no-prelude"))]
pub use prelude::*;