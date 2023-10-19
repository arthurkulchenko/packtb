use std::cell::RefCell;
use std::rc::Rc;
use rand;

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

    pub fn get_level() -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.height {
            n += 1;
        }
        n
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

    pub fn find(&self, offset: u64) -> Option<String> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.height;
                let node = head.head.clone();
                let mut result = None;
                loop {
                    if node.borrow().next[start_level].is_some() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level) {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next)
                                if next.borrow().offset() <= offset =>
                            {
                                n = next.clone();
                            },
                            _ => break,
                        };
                    }
                    if n.borrow().offset == offset {
                        let tmp = n.borrow();
                        result = Some(tmp.value.clone());
                        break;
                    }
                }
                result
            },
            None => None,
        }
    }
}
