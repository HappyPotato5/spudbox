
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

use super::Pattern;

/// Constructs a [`Vec<Recipe<TokenKind>>`](Recipe) in a simplified manner.
/// 
/// Same arguments as [`recipe`] separated by semicolons.
/// 
/// # Example
/// ```ignore
/// let _ = recipe_list![
///     pattern1, kind1;
///     pattern2, kind2, priority2;
///     pattern3, kind3;
///     ...
/// ];
/// ```
#[macro_export]
macro_rules! recipe_list {
    [] => {{ vec![] }};
    [$($pattern: expr, $kind: expr $(, $prio: expr)?);+$(;)?] => {{
        let mut list = Vec::new();
    
        $(list.push(recipe!($pattern, $kind $(, $prio)?)));+;
        
        list
    }};
}
pub use crate::recipe_list;

/// Constructs a [`Recipe`] in a simplified manner.
/// 
/// You'll need to use inference to set `Tokenkind`.
/// 
/// # Examples
/// ```ignore
/// let _ = recipe!(pattern1, kind1);
/// 
/// let _ = recipe!(pattern2, kind2, priority2);
/// ```
#[macro_export]
macro_rules! recipe {
    ($pattern: expr, $kind: expr) => {
        Recipe::new($pattern, $kind)
    };
    ($pattern: expr, $kind: expr, $prio: expr) => {
        Recipe::with_priority($pattern, $kind, $prio)
    };
}
pub use crate::recipe;

// harper:ignore
// <!--vale off-->
//#[allow(unused)]
// macro_rules! count_recipes {
//     () => {0usize};
//     ($_0: expr, $_1: expr $(, $_2: expr)? $(;)?) => { 1usize };
//     ($_0: expr, $_1: expr $(, $_2: expr)?; $($rest: tt)+) => {
//         1usize + count_recipes!($($rest)+)
//     };
// }
//<!--vale on-->

/// A [`Recipe`] contains a [`Pattern`] and a `TokenKind`.
/// 
/// It conveys the idea that a **lexeme** that **matches** the [`pattern`](Pattern) forms a **Token** of a **specific kind** (`TokenKind`).
/// 
/// The [`Lexer`](super::Lexer) uses `priority` to manage conflicts, see [Lexer](super::Lexer) for more.
pub struct Recipe<TokenKind> {
    pub pattern: Box<dyn Pattern>,
    pub priority: u8,
    pub kind: TokenKind,
}

impl<TokenKind> Recipe<TokenKind> {
    /// Constructs a new [`Recipe`] with priority set to 0 by default.
    pub fn new<P: Pattern + 'static>(pattern: P, kind: TokenKind) -> Recipe<TokenKind> {
        Recipe { pattern: Box::new(pattern), kind, priority: 0}
    }

    /// Constructs a new Recipe with the given `priority`.
    pub fn with_priority<P: Pattern + 'static>(pattern: P, kind: TokenKind, priority: u8) -> Recipe<TokenKind> {
        Recipe { pattern: Box::new(pattern), kind, priority}
    }
}