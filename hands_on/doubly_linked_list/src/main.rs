use std::rc::{Rc, Weak};
use core::cell::RefCell;

type NodeRef<T> = Rc<RefCell<Node<T>>>;

pub struct DoublyLinkedList<T> {
    head: Option<NodeRef<T>>,
    tail: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    fn push_front(&mut self, data: T) {
        match self.head.take() {
            Some(node) => {
                // let new_node = Rc::new(RefCell::new(Node { data, next: Some(node.clone), prev: None, })); 
                let new_node = Rc::new(RefCell::new(Node { data, next: Some(node), prev: None, })); 
            },
            None => {
                let new_node = Rc::new(RefCell::new(Node { data, next: None, prev: None, }));
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NodeRef<T>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

fn main() {
    println!("Hello, world!");
}
