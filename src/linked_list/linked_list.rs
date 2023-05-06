use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Rc<RefCell<Node<T>>>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            next: None,
            prev: None,
        }
    }
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push_back(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.tail.take() {
            Some(prev_tail) => {
                prev_tail.borrow_mut().next = Some(Rc::clone(&new_node));
                new_node.borrow_mut().prev = Some(Rc::downgrade(&prev_tail));
            }
            None => {
                self.head = Some(Rc::clone(&new_node));
            }
        }
        self.tail = Some(new_node);
        self.size += 1;
    }

    fn push_front(&mut self, value: T) {
        let new_node = Rc::new(RefCell::new(Node::new(value)));
        match self.head.take() {
            Some(prev_head) => {
                prev_head.borrow_mut().prev = Some(Rc::downgrade(&new_node));
                new_node.borrow_mut().next = Some(prev_head);
            }
            None => {
                self.tail = Some(Rc::clone(&new_node));
            }
        }
        self.head = Some(new_node);
        self.size += 1;
    }

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|prev_tail| {
            match prev_tail.borrow_mut().prev.take() {
                Some(node) => {
                    let cur_tail = node.upgrade().expect("value was destroyed");
                    cur_tail.borrow_mut().next = None;
                    self.tail = Some(cur_tail);
                }
                None => {
                    self.head.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(prev_tail)
                .ok()
                .expect("many strong references")
                .into_inner()
                .value
        })
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|prev_head| {
            match prev_head.borrow_mut().next.take() {
                Some(node) => {
                    node.borrow_mut().prev.take();
                    self.head = Some(node);
                }
                None => {
                    self.tail.take();
                }
            }
            self.size -= 1;
            Rc::try_unwrap(prev_head)
                .ok()
                .expect("many strong references")
                .into_inner()
                .value
        })
    }
}

struct IntoIter<T> {
    iter: LinkedList<T>
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {iter: self}
    }
}

impl<T, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(value: [T; N]) -> Self {
        let mut list = Self::new();
        for i in value {
            list.push_back(i);
        }
        list
    }
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.pop_front()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_back() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_push_front() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(1);
        assert_eq!(list.pop_back(), Some(2));
        assert_eq!(list.pop_back(), Some(1));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_pop_front() {
        let mut list = LinkedList::new();
        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        assert_eq!(list.pop_back(), Some(3));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_into_iter() {
        let list = LinkedList::<usize>::from([0,1,2,3]);
        for (i, v) in list.into_iter().enumerate() {
            assert_eq!(i, v);
        }
    }
}
