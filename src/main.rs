use std::mem;

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

struct Node<T> {
    value: T,
    next: Link<T>,
}

pub struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: Link::Empty }
    }

    pub fn insert(&mut self, element: T) {
        let new_node = Box::new(Node {
            value: element,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn insert_at_tail(&mut self, element: T) {
        let mut tail = &mut self.head;
        while let Link::More(ref mut next) = tail {
            tail = &mut next.next;
        }
        *tail = Link::More(Box::new(Node {
            value: element,
            next: Link::Empty,
        }));
    }

    pub fn insert_at_index(&mut self, index: usize, element: T) {
        if index == 0 {
            return self.insert(element);
        }

        let mut cursor = &mut self.head;
        for _ in 0..index - 1 {
            if let Link::More(ref mut next) = cursor {
                cursor = &mut next.next;
            } else {
                return; // Index out of bounds
            }
        }

        if let Link::More(ref mut next) = cursor {
            let new_node = Box::new(Node {
                value: element,
                next: mem::replace(&mut next.next, Link::Empty),
            });
            next.next = Link::More(new_node);
        }
    }

    pub fn delete(&mut self, index: usize) {
        if index == 0 {
            self.head = match mem::replace(&mut self.head, Link::Empty) {
                Link::More(node) => node.next,
                Link::Empty => Link::Empty,
            };
            return;
        }

        let mut cursor = &mut self.head;
        for _ in 0..index - 1 {
            if let Link::More(ref mut next) = cursor {
                cursor = &mut next.next;
            } else {
                return; // Index out of bounds
            }
        }

        if let Link::More(ref mut node) = cursor {
            *cursor = mem::replace(&mut node.next, Link::Empty);
        }
    }

    pub fn update(&mut self, index: usize, element: T) {
        let mut cursor = &mut self.head;
        for _ in 0..index {
            if let Link::More(ref mut next) = cursor {
                cursor = &mut next.next;
            } else {
                return; // Index out of bounds
            }
        }

        if let Link::More(ref mut node) = cursor {
            node.value = element;
        }
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        let mut cursor = &self.head;
        for _ in 0..index {
            if let Link::More(ref next) = cursor {
                cursor = &next.next;
            } else {
                return None; // Index out of bounds
            }
        }

        if let Link::More(node) = cursor {
            Some(&node.value)
        } else {
            None
        }
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.insert(1);
    list.insert_at_tail(3);
    list.insert_at_index(1, 2);
    println!("{:?}", list.get(1)); // Should print "Some(2)"
    list.update(1, 4);
    println!("{:?}", list.get(1)); // Should print "Some(4)"
    list.delete(1);
    println!("{:?}", list.get(1)); // Should print "Some(3)"
}
