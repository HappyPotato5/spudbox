
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

/// A generational [`Index`] to some element stored in an [`Arena`].
/// 
/// The [`Index`] is generational because it will be **invalidated** after it is used in the [`Arena<T>::free()`] function.
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

/// A memory-contiguous sequence of T elements, where each one can be referenced by it's index.
/// 
/// An arena can be used, for example, to keep nodes of a tree without dealing with references, 
/// And to make memory access faster thanks to memory-contiguity.
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
    /// No allocations will take place until [`Arena<T>::alloc()`] is called.
    pub fn new() -> Arena<T> {
        Arena { slots: Vec::new(), free_slots: Vec::new() }
    }

    fn check_index(&self, idx: Index) -> Option<()> {
        if idx.idx >= self.slots.len() || idx.generation != self.slots[idx.idx].generation { return None; }
        Some(())
    }

    /// Creates a new empty [`Arena<T>`] with enough capacity to hold at least `capacity` elements.
    /// 
    /// `with_capacity()` can be used to make sure no reallocations will take place
    ///  until the number of elements is greater than the given `capacity`.
    pub fn with_capacity(capacity: usize) -> Arena<T> {
        let slots = Vec::with_capacity(capacity);
        let free_slots = Vec::with_capacity(capacity);

        Arena { slots, free_slots }
    }

    /// Returns the **number** of **allocated elements**.
    /// 
    /// Empty slots left after calling [`free()`](Self::free()) won't be counted.
    pub fn len(&self) -> usize {
        self.slots.len() - self.free_slots.len()
    }

    /// Returns the **`capacity`** of the [`Arena<T>`].
    /// 
    /// The `capacity` is the **maximum** amount of elements that can be hold at the same time **without reallocating**.
    pub fn capacity(&self) -> usize {
        self.slots.capacity()
    }

    /// Allocates enough space to hold at least `additional` elements.
    pub fn reserve(&mut self, additional: usize) {
        self.slots.reserve(additional);
        self.free_slots.reserve(additional);
    }

    /// Shrinks the [`Arena`] to fit *at least* it's [`len`](Self::len()).
    /// 
    /// **Only** trailing spaces are removed to **preserve** existing indices.
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

    /// Allocates space for a new element `data`, and returns it's `index`.
    /// 
    /// + This is **O(1)** if the arena has enough capacity to fit the new element.
    /// + **O(n) otherwise**.
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

    /// Frees and returns the element with the given index `idx`.
    /// `idx` becomes invalid after freeing.
    /// 
    /// If the index is invalid None is returned.
    pub fn free(&mut self, idx: Index) -> Option<T> {
        self.check_index(idx)?;

        let slot = &mut self.slots[idx.idx];
        slot.generation += 1;
        self.free_slots.push(idx.idx);

        std::mem::take(&mut slot.data)
    }

    /// Returns a **reference** to the element with the given index `idx`.
    /// 
    /// If the index is invalid `None` is returned.
    pub fn get(&self, idx: Index) -> Option<&T> {
        self.check_index(idx)?;

        self.slots[idx.idx].data.as_ref()
    }

    /// Returns a **mutable reference** to the element with the given index `idx`.
    /// 
    /// If the index is invalid `None` is returned.
    pub fn get_mut(&mut self, idx: Index) -> Option<&mut T> {
        self.check_index(idx)?;

        self.slots[idx.idx].data.as_mut()
    }
}

impl<T> std::ops::Index<Index> for Arena<T> {
    type Output = T;

    /// Returns a reference to the element with the given index `idx`.
    /// 
    /// **Panics** if the index is invalid.
    fn index(&self, index: Index) -> &Self::Output {
        self.get(index).expect("Invalid index")
    }
}

impl<T> std::ops::IndexMut<Index> for Arena<T> {

    /// Returns a mutable reference to the element with the given index `idx`.
    /// 
    /// **Panics** if the index is invalid.
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        self.get_mut(index).expect("Invalid index")
    }
}