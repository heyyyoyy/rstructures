type Node<T> = Option<Box<ListNode<T>>>;

#[derive(Debug)]
struct ListNode<T> {
    value: T,
    next: Node<T>,
}

impl<T> ListNode<T> {
    fn new(value: T, next: Node<T>) -> Self {
        Self { value, next }
    }
}

#[derive(Debug)]
struct SinglyLinkedList<T> {
    head: Node<T>,
    size: usize,
}

impl<T> SinglyLinkedList<T> {
    fn new() -> Self {
        Self {
            head: None,
            size: 0,
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn push(&mut self, value: T) {
        let new_node = Some(Box::new(ListNode::new(value, self.head.take())));
        self.head = new_node;
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.value
        })
    }

    fn get(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn iter(&self) -> Iter<'_, T> {
        Iter {
            head: &self.head,
            size: self.size,
        }
    }

    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            head: self.head.as_deref_mut(),
            size: self.size,
        }
    }
}

struct IntoListIterator<T> {
    iter: SinglyLinkedList<T>,
}

struct Iter<'a, T> {
    head: &'a Node<T>,
    size: usize,
}

struct IterMut<'a, T> {
    head: Option<&'a mut ListNode<T>>,
    size: usize,
}

impl<T> Iterator for IntoListIterator<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.pop()
    }
}

impl<T> IntoIterator for SinglyLinkedList<T> {
    type Item = T;
    type IntoIter = IntoListIterator<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoListIterator { iter: self }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.head.as_ref().map(|node| {
                self.head = &node.next;
                self.size -= 1;
                &node.value
            })
        } else {
            None
        }
    }
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.size > 0 {
            self.head.take().map(|node| {
                self.head = node.next.as_deref_mut();
                self.size -= 1;
                &mut node.value
            })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_push() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.len(), 2);
        assert!(!list.is_empty());
    }

    #[test]
    fn test_pop() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.len(), 2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn test_get() {
        let mut list = SinglyLinkedList::new();
        list.push(1);
        list.push(2);
        assert_eq!(list.get(), Some(&2));
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_into_iter() {
        let mut list = SinglyLinkedList::new();

        for i in 0..10 {
            list.push(i);
        }
        let mut iter = list.into_iter();
        for i in (0..10).rev() {
            assert_eq!(iter.next(), Some(i));
        }
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_iter() {
        let mut list = SinglyLinkedList::new();

        for i in 0..10 {
            list.push(i);
        }
        let mut iter = list.iter();
        for i in (0..10).rev() {
            assert_eq!(iter.next(), Some(&i));
        }
        assert_eq!(iter.next(), None);
        assert_eq!(list.len(), 10);
    }

    #[test]
    fn test_iter_mut() {
        let mut list = SinglyLinkedList::new();

        for i in 0..3 {
            list.push(i);
        }
        for i in list.iter_mut() {
            *i += 10;
        }
        let mut iter = list.iter();
        for i in (0..3).rev() {
            assert_eq!(*iter.next().unwrap(), i + 10);
        }
        assert_eq!(list.len(), 3);
    }
}
