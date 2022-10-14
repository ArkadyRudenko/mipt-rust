#![forbid(unsafe_code)]

use std::{cell::RefCell, collections::VecDeque, fmt::Debug, rc::Rc};
use thiserror::Error;

////////////////////////////////////////////////////////////////////////////////

// TODO: your code goes here.

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
#[error("channel is closed")]
pub struct SendError<T> {
    pub value: T,
}

pub struct Sender<T> {
    // TODO: your code goes here.
}

impl<T> Sender<T> {
    pub fn send(&self, value: T) -> Result<(), SendError<T>> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn is_closed(&self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn same_channel(&self, other: &Self) -> bool {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Self {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Error, Debug)]
pub enum ReceiveError {
    #[error("channel is empty")]
    Empty,
    #[error("channel is closed")]
    Closed,
}

pub struct Receiver<T> {
    // TODO: your code goes here.
}

impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Result<T, ReceiveError> {
        // TODO: your code goes here.
        unimplemented!()
    }

    pub fn close(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        // TODO: your code goes here.
        unimplemented!()
    }
}

////////////////////////////////////////////////////////////////////////////////

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    // TODO: your code goes here.
    unimplemented!()
}
