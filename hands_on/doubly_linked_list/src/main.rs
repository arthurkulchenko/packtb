use std::rc::{Rc, Weak};
use core::cell::RefCell;

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<NextNode<T>>,
    prev: Option<Weak<RefCell<Node<T>>>>,
}

impl<T> Node<T> {
    fn new_list_node(data: T, prev: Option<PrevNode<T>>, next: Option<NextNode<T>>) -> NextNode<T> {
        Rc::new(RefCell::new(Node { data, next, prev }))
    }
}

type NextNode<T> = Rc<RefCell<Node<T>>>;
type PrevNode<T> = Weak<RefCell<Node<T>>>;

#[derive(Debug)]
pub struct DoublyLinkedList<T> {
    head: Option<NextNode<T>>,
    tail: Option<PrevNode<T>>,
}

impl<T> DoublyLinkedList<T> {
    fn new() -> Self {
        DoublyLinkedList { head: None, tail: None }
    }

    fn push(&mut self, data: T) {
        let prev_node = self.tail.take();
        match prev_node {
            Some(node) => {
                let tail_node = Weak::upgrade(&node).unwrap();
                let mut mutable_tail_node = tail_node.borrow_mut();

                let new_node = Node::new_list_node(data, Some(node.clone()), None); // NextNode
                self.tail = Some(Rc::downgrade(&new_node));
                mutable_tail_node.next = Some(new_node);
            },
            None => {
                let new_node = Node::new_list_node(data, None, None); // NextNode
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }

    fn prepend(&mut self, data: T) {
        match self.head.take() {
            Some(head_node) => {
                let mut mutable_head_node = head_node.borrow_mut();

                let new_node = Node::new_list_node(data, None, Some(head_node.clone())); // NextNode
                self.head = Some(new_node.clone());
                mutable_head_node.prev = Some(Rc::downgrade(&new_node));
            },
            None => {
                let new_node = Node::new_list_node(data, None, None); // NextNode
                self.tail = Some(Rc::downgrade(&new_node));
                self.head = Some(new_node);
            }
        }
    }
}

fn main() {
    let mut linked_list = DoublyLinkedList::new();
    linked_list.push(5);
    linked_list.push(2);
    linked_list.prepend(1);
    println!("{:?}", linked_list);
}
