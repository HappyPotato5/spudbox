
use super::{Recipe, RawToken, Pattern, Span, ConflictPolicy};

/// A [`Lexer`] transforms [strings](String) into a sequence of [tokens](RawToken).
/// 
/// The [`Lexer`] stores [recipes](Recipe) that relate *lexemes* that match a [pattern](Pattern) to a *kind* (`TokenKind`).
///
/// # Generics
/// `TokenKind` represents an **enum** of all the possible *token kinds*, **without payload**. (only *unit variants*).
/// 
/// `TokenKind` should derive [`PartialEq`] and [`Clone`].
///
/// # Conflict resolution policy
/// During *tokenization* aka *lexing*, the [`Lexer`] may find more than one matches on the [*patterns*](Pattern) of different [*recipes*](Recipe), so a *conflict* arises.
/// 
/// Conflicts fall into *two categories*:
/// 1. *Different length* matches.
/// 2. *Same length* matches.
/// 
/// For example:
/// 
/// On string `"Where are the potatoes?"`:
/// + **Type-1** conflict for matches:
///     1. `"Where"`
///     2. `"Where are"`
///     3. `"W"`
/// + **Type-2** conflicts for matches:
///     1. "Where"
///     2. "Where"
///     3. "Where"
/// 
/// Each *match* belongs to a *different* [recipe](Recipe).
/// 
/// Note that **all matches start at the same index of the string**.
/// 
/// First, the [`Lexer`] uses [`ConflictPolicy`] to resolve *type-1* conflicts, selecting the longest, or the shortest, *lexeme*.
/// 
/// Then, for *type-2* conflicts the lexer selects the **higher priority** recipe.
/// 
/// In this order **longest > higher priority**.
/// 
/// # Recipes
/// Add or remove [`recipes`](Recipe) by using:
/// + [`Self::add_recipe(recipe)`](Self::add_recipe()).
/// + [`Self::remove_recipe(idx)`](Self::remove_recipe()).
/// 
/// And *accessed* with [`Self::recipes()`].
/// 
/// You can *reserve* space for recipes to *avoid reallocations* with:
/// + The *constructor* [`Self::with_capacity(capacity)`](Self::with_capacity()) pre allocates space for, at least, `capacity` recipes.
/// + The *method* [`Self::reserve(additional)`](Self::reserve()) reserves at least enough capacity for `additional` recipes.
///
/// # Ignore kind
/// + You can make the [`Lexer`] ignore a specific `TokenKind` aka the `IgnoreKind`.
/// 
/// The [`Lexer`] will never add to the *resulting sequence* tokens whose kind with the `IgnoreKind`.
/// 
/// # Examples
/// You can find examples at the [`GitHub repository`](https://github.com/HappyPotato5/spudbox), inside the *examples* folder.
pub struct Lexer<TokenKind: Clone + PartialEq> {
    recipes: Vec<Recipe<TokenKind>>,
    ignore_kind: Option<TokenKind>,
    policy: ConflictPolicy
}

impl<TokenKind: Clone + PartialEq> Lexer<TokenKind> {
    /// Constructs a new [`Lexer`] with an empty recipe list.
    pub fn new(policy: ConflictPolicy) -> Lexer<TokenKind> {
        Lexer { recipes: Vec::new(), ignore_kind: None, policy }
    }

    /// Constructs a new [`Lexer`] with the given `recipes`.
    pub fn with_recipe_list(policy: ConflictPolicy, recipes: Vec<Recipe<TokenKind>>) -> Lexer<TokenKind> {
        Lexer { recipes, ignore_kind: None, policy }
    }

    /// Constructs a new [`Lexer`] with enough capacity for `capacity` recipes.
    pub fn with_capacity(policy: ConflictPolicy, capacity: usize) -> Lexer<TokenKind> {
        Lexer { recipes: Vec::with_capacity(capacity), ignore_kind: None, policy }
    }

    /// Sets the ignore kind to the supplied `Tokenkind`.
    pub fn ignore(&mut self, ignore_kind: TokenKind) {
        self.ignore_kind = Some(ignore_kind);
    }

    /// Stops ignoring the current `IgnoreKind`, if any, and returns it.
    pub fn unignore(&mut self) -> Option<TokenKind> {
        std::mem::take(&mut self.ignore_kind)
    }

    /// Reserves enough space to hold at least `additional` recipes.
    pub fn reserve(&mut self, additional: usize) {
        self.recipes.reserve(additional);
    }

    /// Adds a [`recipe`](Recipe) and returns its index.
    pub fn add_recipe<P: Pattern>(&mut self, recipe: Recipe<TokenKind>) -> usize {
        let res = self.recipes.len();
        self.recipes.push(recipe);
        res
    }

    /// Removes and returns the recipe with the given index.
    /// 
    /// Returns [`None`] if the index falls out of bounds.
    pub fn remove_recipe(&mut self, idx: usize) -> Option<Recipe<TokenKind>> {
        if idx < self.recipes.len() {
            Some(self.recipes.remove(idx))

        } else {None}
    }

    /// Returns a reference to the [`recipe`](Recipe) list.
    pub fn recipes(&self) -> &[Recipe<TokenKind>] {
        &self.recipes
    }

    /// *Tokenizes* the `input` string, with the current recipes, producing a `Vec<RawToken<TokenKind>>`.
    pub fn tokenize(&self, input: &str) -> Result<Vec<RawToken<TokenKind>>, LexerError> {
        let mut remaining = input;
        let mut byte_pos = 0;
        let mut tokens = Vec::new();
        let mut matches = Vec::with_capacity(3);

        let greedy = self.policy == ConflictPolicy::Greedy;
        let policy_cmp = |a, b| if greedy { a > b } else { a < b };

        while !remaining.is_empty() {
            matches.clear();

            for (i, recipe) in self.recipes.iter().enumerate(){
                let len = recipe.pattern.matches(remaining);
                if len == 0 { continue }

                let prio = recipe.priority;

                if matches.is_empty() { matches.push((len, prio, i, &recipe.kind)); continue }

                while let Some((l, p, _, _)) = matches.last()
                    && (policy_cmp(len, *l) || prio > *p) {
                    matches.pop();
                }

                matches.push((len, prio, i, &recipe.kind))
            }

            match matches.len() {
                0 => return Err(LexerError::UnrecognizedCharacter(byte_pos)),
                1 => (),
                _ => return Err(LexerError::RecipeConflict(matches.into_iter().map(|(_, _, idx, _)| idx).collect()))
            }

            let (len, _, _, kind) = matches[0];

            let ignored = self.ignore_kind.as_ref().is_some_and(|x| *x == *kind);

            if !ignored {
                let new_token = RawToken {
                    kind: kind.clone(),
                    lexeme: remaining[..len].to_string(),
                    span: Span {
                        start: byte_pos,
                        end: byte_pos + len - 1,
                    },
                };
                tokens.push(new_token);
            }
            remaining = &remaining[len..];
            byte_pos += len;
        }

        Ok(tokens)
    }
}



/// A [`Lexer`] error.
/// 
/// + [`RecipeConflict`](Self::RecipeConflict): *Two or more recipes* seem valid for the *same lexeme*. 
/// This means that **some recipe needs to have higher priority**.
/// The `vec` contains the *indices of the recipes that conflicted*, and the `usize` the index of the *first char of the conflict*.
/// 
/// + [`UnrecognizedCharacter`](Self::UnrecognizedCharacter): The [`Lexer`] couldn't complete serialization, as some substring didn't match any recipe.
/// The `usize` points to the *first char of the conflict*.
#[derive(Debug, Clone)]
pub enum LexerError {
    RecipeConflict(Vec<usize>),
    UnrecognizedCharacter(usize),
}