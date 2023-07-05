#![doc = include_str!("../README.md")]
use std::ops::{Deref, DerefMut};

/// A struct that stores a `T` and marks if the data inside has been modified.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Dirty<T> {
    /// The data stored in the type. Typically accessed by dereferencing, but can be modified here to avoid being marked as dirty.
    pub data: T,
    /// Whether the data is dirty or not.
    pub dirty: bool,
}

impl<T> Dirty<T> {
    /// Create a new `Clean` value of `t`.
    pub fn new(data: T) -> Self {
        Self { data, dirty: false }
    }

    pub fn clean(&mut self, mut f: impl FnMut(&mut T)) {
        if self.dirty {
            self.dirty = false;
            f(&mut self.data);
        }
    }

    /// Returns a reference to the held value if it is dirty and `None` otherwise.
    pub fn as_dirty(&self) -> Option<&T> {
        if self.dirty {
            Some(&self.data)
        } else {
            None
        }
    }

    /// Returns a reference to the held value if it is clean and `None` otherwise.
    pub fn as_clean(&self) -> Option<&T> {
        if self.dirty {
            None
        } else {
            Some(&self.data)
        }
    }

    /// Returns a mutable reference to the held value if it is dirty and `None` otherwise.
    pub fn as_dirty_mut(&mut self) -> Option<&mut T> {
        if self.dirty {
            Some(&mut self.data)
        } else {
            None
        }
    }

    /// Returns a mutable reference to the held value if it is clean and `None` otherwise.
    pub fn as_clean_mut(&mut self) -> Option<&mut T> {
        if self.dirty {
            None
        } else {
            Some(&mut self.data)
        }
    }
}

impl<T: Default> Default for Dirty<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T> Deref for Dirty<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> DerefMut for Dirty<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.dirty = true;
        &mut self.data
    }
}
