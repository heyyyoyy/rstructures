#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>
}

impl<T> ListNode<T> {
    fn new(value: T, next: Option<Box<ListNode<T>>>) -> Self {
        Self { value, next }
    }
}


#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    size: usize
}


impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        Self { head: None, size: 0 }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push_front(&mut self, value: T) {
        let cur_head = self.head.take();
        let new_node = Some(Box::new(ListNode::new(value, cur_head)));
        self.head = new_node;
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        let cur_node = self.head.take();
        if let Some(node) = cur_node {
            self.head = node.next;
            self.size -= 1;
            Some(node.value)
        } else {
            None
        }
    }
}

struct IntoListIterator<T> {
    iter: SinglyLinkedList<T>
}

impl<T> Iterator for IntoListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.pop_front()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoListIterator {
            iter: self
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop_front() {
        let mut list = SinglyLinkedList::new();
        list.push_front(1);
        list.push_front(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_iteration() {
        let mut list = SinglyLinkedList::new();

        for i in 0..10 {
            list.push_front(i);
        }
        let mut iter = list.into_iter();
        for i in (0..10).rev() {
            assert_eq!(iter.next(), Some(i));
        }
    }
}