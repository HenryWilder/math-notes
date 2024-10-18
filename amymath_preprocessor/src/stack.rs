#![allow(dead_code)]

use std::collections::LinkedList;

/// A collection adapter
pub struct Stack<T>(LinkedList<T>);

impl<T> Stack<T> {
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    pub fn push(&mut self, item: T) {
        self.0.push_front(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn top(&self) -> Option<&T> {
        self.0.front()
    }

    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
    }
}
