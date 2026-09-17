
//! The submodule that handles [`VecSets`](VecSet).

use std::collections::HashSet as Set;
use std::hash::Hash;
use std::fmt::Display;

/// Creates a new [`VecSet<T>`] from the contents of the brackets.
#[macro_export]
macro_rules! vecset {
    [$($es: expr),*] => {{
        use spudbox::vecset::VecSet;

        let v = vec![$($es),*];
        VecSet::from_vec(v)
    }};
}

pub use crate::vecset;

/// A `VecSet` is an ordered set.
/// 
/// Its implemented with an internal HashSet and a Vec, So it takes the double of space as a regular Vec.
/// 
/// When pushing/inserting, elements need to be cloned, which may be very expensive.
/// 
/// # Examples
/// 
/// ```
#[doc = include_str!("../examples/dedup.rs")]
/// ```
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct VecSet<T: Eq + Clone + Hash> {
    set: Set<T>,
    vec: Vec<T>
}

impl<T: Eq + Clone + Hash> VecSet<T> {
    /// Creates a new VecSet<T>
    pub fn new() -> VecSet<T> {
        VecSet { set: Set::new(), vec: Vec::new() }
    }

    /// Creates a [`VecSet<T>`] from a [`Vec<T>`], **removing duplicated** values, while **keeping** the same **order**.
    pub fn from_vec(elements: Vec<T>) -> VecSet<T> {
        let mut res = Self::with_capacity(elements.len());
        res.extend(elements);
        res
    }

    /// Creates a new VecSet<T> with at least the given capacity.
    pub fn with_capacity(capacity: usize) -> VecSet<T> {
        VecSet { set: Set::with_capacity(capacity), vec: Vec::with_capacity(capacity) }
    }

    /// Reserves enough space for at least `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.set.reserve(additional);
        self.vec.reserve(additional);
    }

    /// Pushes a new value to the end of a VecSet.
    /// 
    /// Returns `false` if the element was already in the [`VecSet`], returns `true` otherwise.
    pub fn push(&mut self, value: T) -> bool {
        if self.set.insert(value.clone()) {
            self.vec.push(value);
            return true;
        }
        false
    }

    /// Inserts a new element at the given `index`.
    /// 
    /// The remaining elements are **shifted** if needed.
    /// 
    /// Returns `false` if the element was already in the [`VecSet`], returns `true` otherwise.
    pub fn insert(&mut self, index: usize, value: T) -> bool {
        if self.set.insert(value.clone()) {
            self.vec.insert(index, value);
            return true;
        }
        false
    }

    /// Returns the last element of the [`VecSet`], returns `None` if the [`VecSet`] is empty.
    pub fn pop(&mut self) -> Option<T> {
        let res = self.vec.pop()?;

        self.set.remove(&res);

        Some(res)
    }

    /// Returns the `len` of the [`VecSet`].
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Removes and returns the element with the given `index`.
    /// 
    /// If the `index` is out of bounds `None` is returned.
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        if index < self.len() {
            let res = self.vec.remove(index);
            self.set.remove(&res);
            return Some(res);
        }
        None
    }

    /// Removes and returns the element that fulfills ```element == *value```
    /// 
    /// If the [`VecSet`] doesn't contain such element, `None` is returned.
    pub fn remove_value(&mut self, value: &T) -> Option<T> {
        if self.set.remove(value) {
            let pos = self.vec.iter().position(|x| *x == *value).expect("Broken Invariant");
            return Some(self.vec.remove(pos));
        }
        None
    }

    /// Returns an `iterator` over references of the elements of the [`VecSet`].
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.vec.iter()
    }

    /// Returns whether the [`VecSet`] contains an `element` or not.
    pub fn contains(&self, value: &T) -> bool {
        self.set.contains(value)
    }

    /// Replaces the element in the given `index` by the given `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` if the replacement is successful,
    /// Returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail if the index is **out of bounds** or if the replacement is already in the `[VecSet`].
    pub fn replace_index(&mut self, index: usize, replacement: T) -> Result<T, T> {
        if index >= self.len() || self.contains(&replacement) { return Err(replacement); }

        let el = &mut self.vec[index];
        
        self.set.remove(&*el);
        self.set.insert(replacement.clone());

        Ok(std::mem::replace(el,replacement))
    }

    /// Replaces the element in the given `index` by `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` if the replacement is successful,
    /// Returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail if the value is **not present** in the [`VecSet`] or if the replacement is already in the [`VecSet`].
    pub fn replace_value(&mut self, value: &T, replacement: T) -> Result<T, T> {
        if !self.contains(value) || self.contains(&replacement) { return Err(replacement); }

        let index = self.vec.iter().position(|x| *x == *value).expect("Broken Invariant");
        let el = &mut self.vec[index];
        
        self.set.remove(&*el);
        self.set.insert(replacement.clone());

        Ok(std::mem::replace(el,replacement))
    }

    /// Returns a **reference** to the element with the given `index`, or `None` if the `index` is **out of bounds**.
    /// 
    /// For the panicking version use [`std::ops::Index`].
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }
}

impl<T: Eq + Clone + Hash> std::ops::Index<usize> for VecSet<T> {
    type Output = T;

    /// Returns a **reference** to the element at `index` or panics if the `index` is **out of bounds**.
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Eq + Clone + Hash> IntoIterator for VecSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts the [`VecSet<T>`] into an iterator over T.
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T: Eq + Clone + Hash> IntoIterator for &'a VecSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    /// Returns an iterator over references of the elements in the VecSet.
    /// 
    /// Equivalent of VecSet::iter().
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<T: Display + Eq + Clone + Hash> Display for VecSet<T> {
    /// Displays the VecSet in the form "\[e_0, e_1, e_2 ... e_n-1\]".
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{")?;
        let mut idx = 0usize;
        for i in self {
            std::fmt::Display::fmt(i, f)?;
            if idx < self.len()-1 {
                write!(f, ", ")?;
            }
            idx += 1;
        }
        write!(f, "}}")
    }
}

impl<T: Hash + Eq + Clone> Extend<T> for VecSet<T> {
    /// Appends the contents of the `iter` to the end of the [`VecSet`].
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for el in iter {
            self.push(el);
        }
    }
}