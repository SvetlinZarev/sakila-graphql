use std::ops::{Deref, DerefMut};

pub enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a mut T),
}

impl<'a, T> Deref for MaybeOwned<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        match self {
            MaybeOwned::Owned(x) => x,
            MaybeOwned::Borrowed(x) => **&x,
        }
    }
}

impl<'a, T> DerefMut for MaybeOwned<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        match self {
            MaybeOwned::Owned(x) => x,
            MaybeOwned::Borrowed(x) => *x,
        }
    }
}
