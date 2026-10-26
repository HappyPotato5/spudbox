
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

//! The submodule that handles [`VecSets`](VecSet).

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

/// A simple Ordered Set, implemented with a [`Vec`].
/// 
/// You can use [`insert()`](Self::insert()), [`push()`](Self::push()), [`extend()`](Extend) to add new elements.
/// 
/// And [`pop()`](Self::pop()), [`remove_index()`](Self::remove_index()), [`remove_value()`](Self::remove_value()) to remove elements.
///
/// # Examples
/// ```
#[doc = include_str!("../examples/dedup2.rs")]
/// ```
#[derive(Default, Clone, Debug)]
pub struct VecSet<T: Eq> {
    vec: Vec<T>
}

impl<T: Eq> VecSet<T> {
    /// Creates a new [`VecSet<T>`]
    pub fn new() -> VecSet<T> {
        VecSet { vec: Vec::new() }
    }

    /// Creates a [`VecSet<T>`] from a [`Vec<T>`], **removing duplicated** values, while **keeping** the same **order**.
    pub fn from_vec(elements: Vec<T>) -> VecSet<T> {
        let mut res = Self::with_capacity(elements.len());
        res.extend(elements);
        res
    }

    /// Creates a new [`VecSet<T>`] with at least the given capacity.
    pub fn with_capacity(capacity: usize) -> VecSet<T> {
        VecSet { vec: Vec::with_capacity(capacity) }
    }

    /// Returns whether the [`VecSet<T>`] contains an `element` or not.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn contains(&self, value: &T) -> bool {
        self.vec.iter().any(|x| *x == *value)
    }

    /// Reserves enough space for at least `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.vec.reserve(additional);
    }

    /// Pushes a new value to the end of a [`VecSet<T>`].
    /// 
    /// Returns `Some(value)` for an already present `value`, returns [`None`] otherwise.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn push(&mut self, value: T) -> Option<T> {
        if self.contains(&value) { return Some(value) }
        self.vec.push(value);
        None
    }

    /// Inserts a new element at the given `index`.
    /// 
    /// **Shifts** the remaining elements if needed.
    /// 
    /// Returns `Some(value)` for an already present `value`, returns [`None`] otherwise.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
        if self.contains(&value) { return Some(value) }
        self.vec.insert(index, value);
        None
    }

    /// Returns the last element of the [`VecSet`], returns `None` for an empty [`VecSet`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn pop(&mut self) -> Option<T> {
        self.vec.pop()
    }

    /// Returns the `len` of the [`VecSet`].
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Removes and returns the element with the given `index`.
    /// 
    /// **Shifts** the remaining elements if needed.
    /// 
    /// Returns [`None`] or an **out-of-bounds** index.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        if index >= self.len() { return None }
        Some(self.vec.remove(index))
    }

    /// Removes and returns the element that fulfills ```element == *value```.
    /// 
    /// **Shifts** the remaining elements if needed.
    /// 
    /// If the [`VecSet`] doesn't contain such element, returns [`None`].
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn remove_value(&mut self, value: &T) -> Option<T> {
        let idx = self.iter().position(|x| *x == *value)?;

        Some(self.vec.remove(idx))
    }

    /// Returns an `iterator` over references of the elements of the [`VecSet`].
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.vec.iter()
    }

    /// Replaces the element in the given `index` by the given `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` for a *successful* replacement,
    /// returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail for an **out-of-bounds** index or for an already present `replacement`.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn replace_index(&mut self, index: usize, replacement: T) -> Result<T, T> {
        if self.contains(&replacement) { return Err(replacement) }

        Ok(std::mem::replace(&mut self.vec[index], replacement))
    }

    /// Replaces the element that equals the given `value` by `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` for a successful replacement,
    /// returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail for missing `element` in the [`VecSet`] or for an already present `element`.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn replace_value(&mut self, value: &T, replacement: T) -> Result<T, T> {
        if self.contains(&replacement) { return Err(replacement) }

        let Some(idx) = self.vec.iter().position(|x| *x == *value) else {
            return Err(replacement)
        };

        Ok(std::mem::replace(&mut self.vec[idx], replacement))
    }

    /// Returns a **reference** to the element with the given `index`, returns [`None`] for an **out-of-bounds** `index`.
    /// 
    /// For the panicking version use [`std::ops::Index`].
    /// 
    /// # Time complexity
    /// Constant, O(n).
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    /// Returns the `index` of the element that matches the given `value`.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.vec.iter().position(|x| *x == *value)
    }
}

impl<T: Eq> std::ops::Index<usize> for VecSet<T> {
    type Output = T;

    /// Returns a **reference** to the element at `index`.
    /// 
    /// # Panics
    /// For an **out-of-bounds** index.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Eq> IntoIterator for VecSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts the [`VecSet<T>`] into an iterator over `T`.
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T: Eq> IntoIterator for &'a VecSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    /// Returns an iterator over references of the elements in the [`VecSet`].
    /// 
    /// Same as [`VecSet::iter()`].
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<T: Display + Eq> Display for VecSet<T> {
    /// Displays the [`VecSet`] in the form "\[e_0, e_1, e_2 … e_n-1\]"
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

impl<T: Eq> Extend<T> for VecSet<T> {
    /// Appends the contents of the `iter` to the end of the [`VecSet`].
    /// 
    /// # Time complexity
    /// O(n^2)
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for el in iter {
            self.push(el);
        }
    }
}

impl<T: Eq> PartialEq for VecSet<T> {
    /// Checks that both [`VecSets`](VecSet) contain the **same elements**, in the **same order**.
    /// 
    /// # Time complexity
    /// O(n)
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false
        }

        self.iter().zip(other.iter()).all(|(a, b)| *a == *b)
    }
}