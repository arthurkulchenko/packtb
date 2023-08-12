use std::cell::RefCell;
use std::rc::Rc;

// type Link = Box<Node>;
type Link = Rc<RefCell<Node>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Option<Link>,
}

impl Node {
    fn new(value: String) -> Link {
        Rc::new(RefCell::new(Node { value: value, next: None }))
    }
    // fn drop()
}

// May exceed drop stack size limit
struct TransactionLog {
    head: Option<Link>,
    tail: Option<Link>,
    pub length: u64
}

impl TransactionLog {
    pub fn new_empty() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(last) => last.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone())
        };
        self.length += 1;
        self.tail = Some(new);
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head).ok().expect("Something went wrong").into_inner().value
        })
    }
}

pub struct ListIterator {
    current: Option<Link>,
}

impl ListIterator {
    fn new(start_at: Option<Link>) -> ListIterator {
        ListIterator { current: start_at }
    }
}

impl Iterator for ListIterator {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let current = &self.current;
        let mut result = None;
        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            },
            None => None
        };
        result
    }
}
