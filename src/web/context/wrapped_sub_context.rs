use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use super::WrappedSubContext;

impl<T> Deref for WrappedSubContext<T> {
    type Target = Rc<RefCell<T>>;

    fn deref(&self) -> &Self::Target {
        &self.src
    }
}

impl<T> DerefMut for WrappedSubContext<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.src
    }
}

impl<T> PartialEq for WrappedSubContext<T> {
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation
    }
}

impl<T> Clone for WrappedSubContext<T> {
    fn clone(&self) -> Self {
        Self {
            src: self.src.clone(),
            generation: self.generation.clone(),
        }
    }
}

impl<T> WrappedSubContext<T> {
    pub fn new(value: T) -> Self {
        Self {
            src: Rc::new(RefCell::new(value)),
            generation: 0,
        }
    }
}
