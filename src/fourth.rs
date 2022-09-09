use std::{rc::Rc, cell::RefCell};

pub struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
    prev: Link<T>,
}

impl<T> Node<T> {
    fn new(elem: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            elem,
            next: None,
            prev: None,
        }))
    }
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None, tail: None }
    }
    pub fn push_front(&mut self, elem: T) {
        // New node needs + 2 links, everything else should be +0
        let new_head = Node::new(elem);
        match self.head.take() {
            Some(old_head) => {
                // non empty list, need to connect the old_head
                old_head.borrow_mut().prev = Some(new_head.clone()); // +1 new head
                new_head.borrow_mut().next = Some(old_head);         // + 1 old_head
                self.head = Some(new_head);             // +1 new_head, -1 old_head
                // Total link changes +2 new head, +1 old head
            }
            None => {
                // Empty list, need to set the tail
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
                // Total changes: +2 new_head
            }
        }
    }
    pub fn pop_front(&mut self) -> Option<T> {
        // need to take the old head, ensure its -2 links to old head
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    // we are not emptying the list
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                    // total: -2 old (2x take) +0 new head
                }
                None => {
                    // emptying list
                    self.tail.take();
                    // Total: -2 old, no new
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Pop nodes until None is returned (empty list)
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // check empty list behaves correctly
        assert_eq!(list.pop_front(), None);

        // Populate list
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));

        // Push some more to make sure nothing is corrupted
        list.push_front(4);
        list.push_front(5);

        // Check normal removal
        assert_eq!(list.pop_front(), Some(5));
        assert_eq!(list.pop_front(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
