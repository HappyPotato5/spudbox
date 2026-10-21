
/// A [`Span`] holds the starting and ending positions (**in bytes**) of the token in the source string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: usize,
    pub end: usize
}