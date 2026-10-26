
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

//! The submodule that handles [`Arenas`](Arena).

#[derive(Clone, Debug)]
struct Slot<T> {
    data: Option<T>,
    generation: usize
}

impl<T> Slot<T> {
    fn with_data(data: T) -> Slot<T> {
        Slot { data: Some(data), generation: 0 }
    }
}

/// A *generational* [`Index`] to some element stored in an [`Arena`].
/// 
/// A generational index will get **invalidated as soon as freed** in [`Arena::free()`].
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub struct Index {
    idx: usize,
    generation: usize,
}

impl Index {
    fn new(idx: usize, generation: usize) -> Index {
        Index { idx, generation }
    }
}

/// A *memory-contiguous* sequence of `T` elements, where you can use their [`indices`](Index) to reference them.
/// 
/// You can use an [`Arena`], for example, to keep nodes of a *tree* without dealing with references, 
/// And to make memory access **faster** thanks to *memory-contiguity*.
/// 
/// # Examples
/// ```
#[doc = include_str!("../examples/tree.rs")]
/// ```
/// Indexes are given with the [`Arena<T>::alloc()`] function, and stay valid until [`Arena<T>::free()`] is called.
#[derive(Clone, Debug)]
pub struct Arena<T> {
    slots: Vec<Slot<T>>,
    free_slots: Vec<usize>,
}

impl<T> Arena<T> {
    /// Creates a new empty [`Arena<T>`].
    /// 
    /// No allocations will take place until the use of [`Arena<T>::alloc()`].
    pub fn new() -> Arena<T> {
        Arena { slots: Vec::new(), free_slots: Vec::new() }
    }

    fn check_index(&self, idx: Index) -> Option<()> {
        if idx.idx >= self.slots.len() || idx.generation != self.slots[idx.idx].generation { return None; }
        Some(())
    }

    /// Creates a new empty [`Arena<T>`] with enough capacity to hold at least `capacity` elements.
    /// 
    /// No *reallocations* will take place until the number of elements *exceeds* the supplied `capacity`.
    pub fn with_capacity(capacity: usize) -> Arena<T> {
        let slots = Vec::with_capacity(capacity);
        let free_slots = Vec::with_capacity(capacity);

        Arena { slots, free_slots }
    }

    /// Returns the **number** of **allocated elements**.
    /// 
    /// Doesn't count empty slots left after calling [`free()`](Self::free()).
    pub fn len(&self) -> usize {
        self.slots.len() - self.free_slots.len()
    }

    /// Returns the **`capacity`** of the [`Arena<T>`].
    /// 
    /// The *total* amount of elements it can hold a the *same time*.
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Allocates enough space to hold at least `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.slots.reserve(additional);
        self.free_slots.reserve(additional);
    }

    /// Shrinks the [`Arena`] to fit *at least* its [`length`](Self::len()).
    /// 
    /// **Only** removes trailing spaces to **preserve** existing [indices](Index).
    pub fn shrink_to_fit(&mut self) {
        loop {
            let Some(slot) = self.slots.last() else { break };

            if slot.data.is_some() { break }

            self.slots.pop();
        }

        self.slots.shrink_to_fit();

        let mut free_slots = Vec::with_capacity(self.slots.capacity());
        for i in (0..self.slots.len()).rev() {
            if self.slots[i].data.is_none() {
                free_slots.push(i);
            }
        }

        self.free_slots = free_slots;
    }

    /// Allocates space for a new element `data`, and returns its [`index`](Index).
    /// 
    /// # Time complexity
    /// Amortized O(1).
    /// Worst case: O(n).
    pub fn alloc(&mut self, data: T) -> Index {
        if let Some(idx) = self.free_slots.pop() {
            let slot = &mut self.slots[idx];
            slot.data = Some(data);

            Index::new(idx, slot.generation)
        } else {
            let new_slot = Slot::with_data(data);
            
            let idx = self.slots.len();
            self.slots.push(new_slot);

            Index::new(idx, 0)
        }
    }

    /// Frees and returns the element with the given [index](Index) `idx`.
    /// Then `idx` gets *invalidated*.
    /// 
    /// For an invalid [index](Index), returns [`None`].
    pub fn free(&mut self, idx: Index) -> Option<T> {
        self.check_index(idx)?;

        let slot = &mut self.slots[idx.idx];
        slot.generation += 1;
        self.free_slots.push(idx.idx);

        std::mem::take(&mut slot.data)
    }

    /// Returns a **reference** to the element with the given [index](Index) `idx`.
    /// 
    /// For an invalid [index](Index), returns [`None`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn get(&self, idx: Index) -> Option<&T> {
        self.check_index(idx)?;

        self.slots[idx.idx].data.as_ref()
    }

    /// Returns a **mutable reference** to the element with the given index `idx`.
    /// 
    /// For an invalid [index](Index), returns [`None`].
    /// 
    /// # Time complexity
    /// Constant, O(1).
    pub fn get_mut(&mut self, idx: Index) -> Option<&mut T> {
        self.check_index(idx)?;

        self.slots[idx.idx].data.as_mut()
    }

    /// Applies the closure `f` to every contained `T` element, and returns the resulting [`Arena<O>`].
    /// 
    /// The [`indices`](Index) don't get invalidated after this operation.
    /// 
    /// # Time complexity
    /// Lineal, O(n).
    pub fn map<O, F>(self, mut f: F) -> Arena<O>
        where F: FnMut(T) -> O
    {
        let mut slots = Vec::with_capacity(self.capacity());

        for slot in self.slots {
            slots.push(Slot {
                data: slot.data.map(&mut f),
                generation: slot.generation,
            });
        }

        let mut res = Arena::new();

        res.slots = slots;
        res.free_slots = self.free_slots;
        res
    }
}

impl<T> std::ops::Index<Index> for Arena<T> {
    type Output = T;

    /// Returns a *reference* to the element with the given index `idx`.
    /// 
    /// # Panics
    /// **Panics** for an invalid index.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).expect("Invalid index")
    }
}

impl<T> std::ops::IndexMut<Index> for Arena<T> {

    /// Returns a *mutable reference* to the element with the given [index](Index) `idx`.
    /// 
    /// # Panics
    /// **Panics** for an invalid index.
    /// 
    /// # Time complexity
    /// Constant, O(1).
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.get_mut(index).expect("Invalid index")
    }
}