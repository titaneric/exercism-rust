use std::iter::FromIterator;
use std::mem;

pub struct SimpleLinkedList<T> {
    _len: usize,
    head: Option<Box<Node<T>>>,
}
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            _len: 0,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self._len == 0
    }

    pub fn len(&self) -> usize {
        self._len
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });
        self.head = Some(new_node);
        self._len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().and_then(|node| {
            self.head = node.next;
            self._len -= 1;
            Some(node.data)
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().and_then(|node| Some(&node.data))
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut cur = self.head;
        let mut target = SimpleLinkedList::new();
        while let Some(node) = cur {
            target.push(node.data);
            cur = node.next;
        }
        target
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut result = Self::new();
        for data in _iter.into_iter(){
            result.push(data);
        }
        result
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut cur = self.rev().head;
        let mut results = vec![];
        while let Some(node) = cur {
            results.push(node.data);
            cur = node.next;
        }
        results
    }
}
