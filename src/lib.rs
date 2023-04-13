use std::{
    mem::MaybeUninit,
    ops::{Deref, DerefMut},
};

#[derive(Debug)]
pub enum Dirty<T> {
    Clean(T),
    Dirty(T),
}

impl<T> Dirty<T> {
    pub fn new(t: T) -> Self {
        Self::Clean(t)
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
            // SAFETY: This is safe since the zero value is dropped immediately
            // after this block is over.
            let zeroed = unsafe { MaybeUninit::zeroed().assume_init() };
            *self = Self::Dirty(std::mem::replace(t, zeroed));
        }

        if let Self::Dirty(t) = self {
            t
        } else {
            unreachable!()
        }
    }
}
