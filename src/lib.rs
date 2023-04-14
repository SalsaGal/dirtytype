use std::ops::{Deref, DerefMut};

#[derive(Debug, PartialEq, Eq)]
pub enum Dirty<T> {
    Clean(T),
    Dirty(T),
}

impl<T: Default> Dirty<T> {
    pub fn new(t: T) -> Self {
        Self::Clean(t)
    }

    pub fn clear(&mut self) {
        if let Self::Dirty(t) = self {
            *self = Self::Clean(std::mem::take(t));
        }
    }

    pub fn is_clean(&self) -> bool {
        matches!(self, Self::Clean(..))
    }

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
            unreachable!()
        }
    }
}
