
/// The [`Lexer`](super::Lexer) use the [`ConflictPolicy`] to resolve *type-1 conflicts*.
/// 
/// **Type-1 conflicts** occur when *different recipes* match *different length lexemes*.
/// 
/// + [`Greedy`](Self::Greedy): Selects the [recipe](super::Recipe) that matched the *longest* lexeme.
/// + [`Lazy`](Self::Lazy): Selects the [recipe](super::Recipe) that matched the *shortest* lexeme.
/// 
/// If the lexemes have the same length then the [`Lexer`](super::Lexer) uses the [`priority`](super::Recipe::priority) of the recipes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictPolicy {
    Greedy,
    Lazy
}



/// A [`Pattern`] can match part of a [`String`], producing a substring.
/// 
/// [`String`] and `&str` derive [`Pattern`] by default, and they will match any substring **exactly equal** to themselves.
pub trait Pattern {
    /// Should return the **length of the match** for this [`Pattern`] (in bytes).
    /// 
    /// Takes the `remaining` string.
    /// 
    /// This [`Pattern`] may perform **greedy** or **lazy** matching, *depending of your implementation*.
    fn matches(&self, remaining: &str) -> usize;
}

impl Pattern for String {
    fn matches(&self, chars: &str) -> usize {
        if chars.starts_with(self) {
            self.len()
        } else { 0 }
    }
}

impl Pattern for str {
    fn matches(&self, chars: &str) -> usize {
        if chars.starts_with(self) {
            self.len()
        } else { 0 }
    }
}

impl Pattern for &str {
    fn matches(&self, chars: &str) -> usize {
        <str as Pattern>::matches(self,chars)
    }
}