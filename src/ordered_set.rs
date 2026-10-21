
//! The submodule that handles [`OrderedSets`](OrderedSet).

use std::hash::{DefaultHasher, Hash, Hasher};
use std::fmt::Display;
use hashbrown::hash_table::HashTable;

/// Creates a new [`OrderedSet<T>`] from the contents of the brackets.
#[macro_export]
macro_rules! ordset {
    [$($es: expr),*] => {{
        use spudbox::ordered_set::OrderedSet;

        let v = vec![$($es),*];
        OrderedSet::from_vec(v)
    }};
}

pub use crate::ordset;

/// A simple and fast `OrderedSet`.
/// 
/// You can use [`insert()`](Self::insert()), [`push()`](Self::push()), [`exptend()`](Extend) to add new elements.
/// 
/// And [`pop()`](Self::pop()), [`remove_index()`](Self::remove_index()), [`remove_value()`](Self::remove_value()) to remove elements.
///
/// # Examples
/// ```
#[doc = include_str!("../examples/dedup.rs")]
/// ```
#[derive(Default, Clone, Debug)]
pub struct OrderedSet<T: Eq + Hash> {
    table: HashTable<usize>,
    vec: Vec<T>
}

impl<T: Eq + Hash> OrderedSet<T> {
    /// Creates a new [`OrderedSet<T>`]
    pub fn new() -> OrderedSet<T> {
        OrderedSet { table: HashTable::new(), vec: Vec::new() }
    }

    /// Creates a [`OrderedSet<T>`] from a [`Vec<T>`], **removing duplicated** values, while **keeping** the same **order**.
    /// 
    /// # Time complexity
    /// O(n)
    pub fn from_vec(elements: Vec<T>) -> OrderedSet<T> {
        let mut res = Self::with_capacity(elements.len());
        res.extend(elements);
        res
    }

    /// Creates a new [`OrderedSet<T>`] with at least the given capacity.
    pub fn with_capacity(capacity: usize) -> OrderedSet<T> {
        OrderedSet { table: HashTable::with_capacity(capacity), vec: Vec::with_capacity(capacity) }
    }

    fn hash<V: Hash>(value: &V) -> u64 {
        let mut hasher = DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }

    /// Returns whether the [`OrderedSet<T>`] contains an `element` or not.
    /// 
    /// # Time complexity
    /// O(1)
    pub fn contains(&self, value: &T) -> bool {
        self.table.find(Self::hash(value), |x| self.vec[*x] == *value).is_some()
    }

    /// Reserves enough space for at least `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.table.reserve(additional, |x| Self::hash(&self.vec[*x]));
        self.vec.reserve(additional);
    }

    /// Pushes a new value to the end of a [`OrderedSet<T>`].
    /// 
    /// Returns `Some(value)` if the [`OrderedSet`] already contains the `value`.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn push(&mut self, value: T) -> Option<T> {
        if self.contains(&value) { return Some(value) }

        self.table.insert_unique(Self::hash(&value), self.vec.len(), Self::hash);
        self.vec.push(value);
        
        None
    }

    /// Inserts a new element at the given `index`.
    /// 
    /// Also shifts the elements after `index` if needed.
    /// 
    /// Returns `Some(value)` if the [`OrderedSet<T>`] already contains `value`, returns [`None`] otherwise.
    /// 
    /// # Time complexity
    /// Lineal, O(n).
    pub fn insert(&mut self, index: usize, value: T) -> Option<T> {
        if index > self.vec.len() || self.contains(&value) { return Some(value) }

        for i in self.table.iter_mut() {
            if *i >= index {
                *i += 1;
            }
        }

        self.table.insert_unique(Self::hash(&value), index, Self::hash);
        self.vec.insert(index, value);
        
        None
    }

    /// Returns the last element of the [`OrderedSet`], for an empty [`OrderedSet`] returns [`None`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn pop(&mut self) -> Option<T> {
        if self.vec.is_empty() { return None }
        let idx = self.vec.len() - 1;

        let res = self.vec.last()?;

        let Ok(occ) = self.table.find_entry(Self::hash(res), |x| *x == idx) else {
            return None
        };
        occ.remove();

        self.vec.pop()
    }

    /// Returns the `len` of the [`OrderedSet`].
    /// 
    /// # Time complexity
    /// O(1)
    pub fn len(&self) -> usize {
        self.vec.len()
    }

    /// Removes and returns the element with the given `index`.
    /// 
    /// For an out-of-bounds index, returns [`None`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn remove_index(&mut self, index: usize) -> Option<T> {
        if index >= self.len() { return None }

        let res = &self.vec[index];

        let Ok(occ) = self.table.find_entry(Self::hash(res), |x| self.vec[*x] == *res) else {
            return None
        };
        occ.remove();

        for i in self.table.iter_mut() {
            if *i > index {
                *i -= 1;
            }
        }

        Some(self.vec.remove(index))
    }

    /// Removes and returns the element that fulfills ```element == *value```.
    /// 
    /// If no such element exists, returns [`None`].
    /// 
    /// Also shifts the following elements if needed.
    /// 
    /// # Time complexity
    /// Lineal, O(n).
    pub fn remove_value(&mut self, value: &T) -> Option<T> {
        let Ok(occ) = self.table.find_entry(Self::hash(value), |x| self.vec[*x] == *value) else {
            return None;
        };
        let (idx, _) = occ.remove();

        for i in self.table.iter_mut() {
            if *i > idx {
                *i -= 1;
            }
        }

        Some(self.vec.remove(idx))
    }

    /// Returns an `iterator` over references of the elements of the [`OrderedSet`].
    pub fn iter(&self) -> core::slice::Iter<'_, T> {
        self.vec.iter()
    }

    /// Replaces the element in the given `index` by the given `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` for a successful *replacement*,
    /// returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail for an **out-of-bounds** index or for an **already present** replacement.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn replace_index(&mut self, index: usize, replacement: T) -> Result<T, T> {
        if index >= self.len() || self.contains(&replacement) { return Err(replacement) }

        let hash = Self::hash(&self.vec[index]);
        let Ok(occ) = self.table.find_entry(hash, |x| *x == index) else {
            return Err(replacement)
        };
        occ.remove();

        self.table.insert_unique(Self::hash(&replacement), index, Self::hash);
        
        Ok(std::mem::replace(&mut self.vec[index], replacement))
    }

    /// Replaces the element that equals the given `value` by `replacement`.
    /// 
    /// Returns `Ok(replaced_element)` for a successful *replacement*,
    /// returns `Err(replacement)` otherwise.
    /// 
    /// The replacement will fail for missing `value` or for an **already present** `replacement`.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn replace_value(&mut self, value: &T, replacement: T) -> Result<T, T> {
        if !self.contains(value) || self.contains(&replacement) { return Err(replacement); }

        let Ok(occ) = self.table.find_entry(Self::hash(value), |x| self.vec[*x] == *value) else {
            return Err(replacement)
        };
        let (index, _) = occ.remove();

        self.table.insert_unique(Self::hash(&replacement), index, Self::hash);
        
        Ok(std::mem::replace(&mut self.vec[index], replacement))
    }

    /// Returns a **reference** to the element with the given `index`, returns [`None`] for an **out-of-bounds** `index`.
    /// 
    /// For the panicking version use [`std::ops::Index`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn get(&self, index: usize) -> Option<&T> {
        self.vec.get(index)
    }

    /// Returns the `index` of the element that matches the given `value`.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn get_index(&self, value: &T) -> Option<usize> {
        self.table.find(Self::hash(value), |x| self.vec[*x] == *value).map(|x| *x)
    }
}

impl<T: Eq + Clone + Hash> std::ops::Index<usize> for OrderedSet<T> {
    type Output = T;

    /// Returns a **reference** to the element at the given `index`.
    /// 
    /// # Panics
    /// Panics for **an out-of-bounds** `index`.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    fn index(&self, index: usize) -> &Self::Output {
        &self.vec[index]
    }
}

impl<T: Eq + Hash> IntoIterator for OrderedSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    /// Converts the [`OrderedSet<T>`] into an iterator over `T`.
    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
    }
}

impl<'a, T: Eq + Hash> IntoIterator for &'a OrderedSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    /// Returns an iterator over references of the elements in the [`OrderedSet`].
    /// 
    /// Same as [`OrderedSet::iter()`].
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
    }
}

impl<T: Display + Eq + Hash> Display for OrderedSet<T> {
    /// Displays the [`OrderedSet`] in the form "\[e_0, e_1, e_2 … e_n-1\]"
    /// 
    /// # Time complexity
    /// Linear, O(n).
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

impl<T: Hash + Eq> Extend<T> for OrderedSet<T> {
    /// Appends the contents of the `iter` to the end of the [`OrderedSet`].
    /// 
    /// # Time complexity
    /// Linear, O(n).
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for el in iter {
            self.push(el);
        }
    }
}

impl<T: Eq + Hash> PartialEq for OrderedSet<T> {
    /// Checks that both [`OrderedSets`](OrderedSet) contain the **same elements**, in the **same order**.
    /// 
    /// # Time complexity
    /// Linear, O(n).
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false
        }

        self.iter().zip(other.iter()).all(|(a, b)| *a == *b)
    }
}

impl<T: Eq + Hash> Eq for OrderedSet<T> {}