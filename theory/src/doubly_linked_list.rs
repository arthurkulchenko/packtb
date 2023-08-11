use std::cell::RefCell;
use std::rc::Rc;

// type Link = Box<Node>;
type Link = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
struct Node {
    value: String,
    next: Option<Link>,
    prew: Option<Link>,
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
    pub fn init() -> TransactionLog {
        TransactionLog { head: None, tail: None, length: 0 }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);
        match self.tail.take() {
            // Why do I need to borrow here using borrow_mut method?
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
    // Node links type
    current: Option<Link>,
}

impl ListIterator {
    fn new(start_at: Option<Link>) -> ListIterator {
        ListIterator { current: start_at }
    }
}

// Updates current and returns value
impl Iterator for ListIterator {
    // type Item = String;
    //                       Node value
    fn next(&mut self) -> Option<String> {
        let mut result = None;
        // Set current to "next"
        // Why do I need reference here?
        self.current = match &self.current {
            // element is a reference or a value of Node type?
            Some(ref element) => {
                // Why do I need to borrow here using "borrow" method?
                let element = element.borrow();
                result = Some(element.value.clone());
                element.next.clone()
            },
            None => None
        };
        result
    }
}

impl DoudleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<String> {
        let mut result = None;
        self.current = match &self.current {
            Some(ref element) => {
                let element = element.borrow();
                result = Some(element.value.clone());
                element.prew.clone()
            },
            None => None
        };
        result
    }
}
