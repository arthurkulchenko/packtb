use std::cell::RefCell;
use std::rc::Rc;

type Link = Rc<RefCell<Node>>;

#[derive(Debug, Clone)]
struct Node {
    pub value: String,
    next: Vec<Option<Link>>,
    offset: u64,
}

struct SkipList {
    head: Option<Link>,
    tails: Vec<Option<Link>>,
    length: usize,
    height: u64,
}

impl SkipList {
    pub fn new() -> SkipList {
        SkipList { head: None, tails: vec![None], length: 0, height: 0 }
    }

    pub fn insert(&mut self, value: String, offset: u64) {
        let level = 1 + if self.head.is_none() {
            self.height
        } else {
            self.get_level()
        }

        let new = Node { value: value, next: vec![None; level], offset: offset };

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next();
                next[i] = Some(new.clone());
            }
            self.tails[i] = Some(new.clone());
        }

        if self.head.is_none() {
            self.head = Some(new.clone());
        }

        self.length += 1;
    }
    // remove
    // find_by_value
}
