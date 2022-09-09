pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        // Take the head value and replace with empty value
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            // While the current link in the iteration is another Node,
            // replace the boxed node with Empty
            cur_link = boxed_node.next.take();
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's next field has been set to Link::Empty
            // so no unbounded recursion happens
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        
        // check Empty list behaves correctly
        assert_eq!(list.pop(), None);

        // Populate a list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push more to ensure list structure is still intact
        list.push(4);
        list.push(5);

        // Check removal again
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion of list elements
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
