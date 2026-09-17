
#![doc = include_str!("../README.md")]

#[cfg(feature = "arena")]
pub mod arena;

#[cfg(feature = "vecset")]
pub mod vecset;

pub mod prelude {
    use super::*;

    #[cfg(feature = "arena")]
    pub use arena::*;

    #[cfg(feature = "vecset")]
    pub use vecset::*;
}

#[cfg(not(feature = "no-prelude"))]
pub use prelude::*;