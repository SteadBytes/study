//! # A Bad Stack
use std::mem;

pub struct List {
    head: Link,
}

// Note: This is a essentially a poor implementation of `Option`. See `second.rs`.
enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            // Replace `self.head` with `Link::Empty` *before* replacing it with the head of the
            // list. This is to avoid leaving `self` partially initialized due to moving
            // `self.head` into `new_node`.
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // Mutable reference to `self` -> to move items they must by *replaced*, not just *removed*
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        // Iteratate through list, hoisting nodes out of their `Box` to safely drop their contents.
        // Note that this could be achieved with `while let Some(_) = self.pop() {}`, however doing
        // so moves the *values* stored within the nodes whereas this implementation only
        // manipulates pointers between nodes. For `i32` this is likely a premature optimization,
        // however for larger types this would be a significant overhead.
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            // `next` field set to `Link::Empty` -> prevent unbounded recursion when dropping
            // `boxed_node`
            // - Otherwise, to drop the Node, `next` must be dropped but to drop the `next` node
            // it's `next` must be dropped but to drop ...
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // `boxed_node` out of scope here -> dropped
        }
    }
}

#[cfg(test)]
mod tests {

    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
