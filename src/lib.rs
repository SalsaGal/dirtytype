use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Dirty<T> {
    Clean(T),
    Dirty(T),
}

impl<T> Dirty<T> {
    pub fn new(t: T) -> Self {
        Self::Clean(t)
    }

    pub fn clear(&mut self) {
        if let Self::Dirty(t) = self {
            *self = Self::Clean(replace_zero(t))
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
            Self::Clean(t) => t,
            Self::Dirty(t) => t,
        }
    }
}

impl<T> DerefMut for Dirty<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if let Self::Clean(t) = self {
            *self = Self::Dirty(replace_zero(t));
        }

        if let Self::Dirty(t) = self {
            t
        } else {
            unreachable!()
        }
    }
}

fn replace_zero<T>(t: &mut T) -> T {
    // SAFETY: This is safe since the zero value is dropped immediately
    // after this block is over.
    let zeroed = unsafe { MaybeUninit::zeroed().assume_init() };
    std::mem::replace(t, zeroed)
}
