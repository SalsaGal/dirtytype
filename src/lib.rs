#![doc = include_str!("../README.md")]
use std::ops::{Deref, DerefMut};

/// A struct that stores a `T` and marks if the data inside has been modified.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dirty<T> {
    /// The type that has not been modified, or has had the `Dirty` flag cleared.
    Clean(T),
    /// The type that has been modified.
    Dirty(T),
}

impl<T: Default> Dirty<T> {
    /// Create a new `Clean` value of `t`.
    pub fn new(t: T) -> Self {
        Self::Clean(t)
    }

    /// Returns a reference to the held value if it is dirty and `None` otherwise. Calling this cleans the value as it is interpreted as handling dirtiness.
    pub fn dirty(&mut self) -> Option<&T> {
        match self {
            Self::Clean(..) => None,
            Self::Dirty(..) => {
                self.clear();
                Some(&**self)
            }
        }
    }

    /// Returns a reference to the held value if it is clean and `None` otherwise.
    pub fn clean(&mut self) -> Option<&T> {
        match self {
            Self::Clean(..) => Some(&**self),
            Self::Dirty(..) => None,
        }
    }

    /// Sets the value to `Clean`.
    pub fn clear(&mut self) {
        if let Self::Dirty(t) = self {
            *self = Self::Clean(std::mem::take(t));
        }
    }

    /// Sets the value to `Dirty`.
    pub fn mark(&mut self) {
        if let Self::Clean(t) = self {
            *self = Self::Dirty(std::mem::take(t));
        }
    }

    /// Sets the value to the opposite of its current value.
    pub fn invert(&mut self) {
        match self {
            Self::Clean(..) => self.mark(),
            Self::Dirty(..) => self.clear(),
        }
    }

    /// Returns if the value is clean.
    pub fn is_clean(&self) -> bool {
        matches!(self, Self::Clean(..))
    }

    /// Returns if the value is dirty.
    pub fn is_dirty(&self) -> bool {
        matches!(self, Self::Dirty(..))
    }
}

impl<T: Default> Default for Dirty<T> {
    fn default() -> Self {
        Self::Clean(T::default())
    }
}

impl<T> Deref for Dirty<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        match self {
            Self::Clean(t) | Self::Dirty(t) => t,
        }
    }
}

impl<T: Default> DerefMut for Dirty<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Self::Clean(t) = self {
            *self = Self::Dirty(std::mem::take(t));
        }

        if let Self::Dirty(t) = self {
            t
        } else {
            // self is set to `Dirty` if it's not already
            unreachable!()
        }
    }
}
