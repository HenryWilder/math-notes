#![allow(dead_code)]

use std::collections::LinkedList;

/// A collection adapter for [`LinkedList`].
///
/// Allow pushing/popping from the top in O(1) time. All other elements are hidden.
pub struct Stack<T>(LinkedList<T>);

impl<T> Stack<T> {
    /// Construct an empty stack.
    pub fn new() -> Self {
        Self(LinkedList::new())
    }

    /// Add an item to the top of the stack.
    pub fn push(&mut self, item: T) {
        self.0.push_front(item);
    }

    /// Remove the item at the top and returns it, or `None` if the list is empty.
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop_front()
    }

    /// Returns `true` if the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// A reference to the item at the top of the stack, or `None` of the stack is empty.
    pub fn top(&self) -> Option<&T> {
        self.0.front()
    }

    /// A mutable reference to the item at the top of the stack, or `None` of the stack is empty.
    pub fn top_mut(&mut self) -> Option<&mut T> {
        self.0.front_mut()
    }
}
