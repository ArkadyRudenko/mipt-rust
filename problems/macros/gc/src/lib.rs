#![forbid(unsafe_code)]

pub use gc_derive::Scan;

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    marker::PhantomData,
    ops::Deref,
    rc::{Rc, Weak},
};

////////////////////////////////////////////////////////////////////////////////

pub struct Gc<T> {
    weak: Weak<T>,
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            weak: self.weak.clone(),
        }
    }
}

impl<T> Gc<T> {
    pub fn borrow(&self) -> GcRef<'_, T> {
        // TODO: your code goes here.
        unimplemented!()
    }
}

pub struct GcRef<'a, T> {
    rc: Rc<T>,
    lifetime: PhantomData<&'a Gc<T>>,
}

impl<'a, T> Deref for GcRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.rc
    }
}

////////////////////////////////////////////////////////////////////////////////

pub trait Scan {
    // TODO: your code goes here.
}

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

pub struct Arena {
    // TODO: your code goes here.
}

impl Arena {
    pub fn new() -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn allocation_count(&self) -> usize {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn alloc<T: Scan + 'static>(&mut self, obj: T) -> Gc<T> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn sweep(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }

    // TODO: your code goes here.
}
