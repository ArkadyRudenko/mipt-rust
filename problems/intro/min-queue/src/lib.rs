#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::cmp;

#[derive(Default)]
pub struct MinQueue<T> {
    stack1: VecDeque<(T, T)>,
    stack2: VecDeque<(T, T)>,
    size: usize,
}

impl<T: Clone + Ord> MinQueue<T> {
    pub fn new() -> Self {
        Self {
            stack1: VecDeque::default(),
            stack2: VecDeque::default(),
            size: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        self.size += 1;
        let min = if self.stack1.is_empty() {
            val.clone()
        } else {
            cmp::min(val.clone(), self.stack1.back().unwrap().1.clone()) // TODO remove .clone()
        };
        self.stack1.push_back((val, min));
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        self.size -= 1;
        if self.stack2.is_empty() {
            while !self.stack1.is_empty() {
                let element = self.stack1.pop_back().unwrap().0;

                let minimum = if self.stack2.is_empty() {
                    element.clone()
                } else {
                    cmp::min(element.clone(), self.stack2.back().unwrap().1.clone())
                };

                self.stack2.push_back((element, minimum));
            }
        }

        Some(self.stack2.pop_back().unwrap().0)
    }

    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        if self.stack1.is_empty() && !self.stack2.is_empty() {
            return Some(&self.stack2.back().unwrap().0);
        }
        Some(&self.stack1.front().unwrap().0)
    }

    pub fn min(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }
        return if self.stack1.is_empty() || self.stack2.is_empty() {
            if self.stack1.is_empty() {
                Some(&(self.stack2.back().unwrap().1))
            } else {
                Some(&(self.stack1.back().unwrap().1))
            }
        } else {
            if self.stack1.back().unwrap().1 < self.stack2.back().unwrap().1 {
                Some(&self.stack1.back().unwrap().1)
            } else {
                Some(&self.stack2.back().unwrap().1)
            }
        };
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }
}
