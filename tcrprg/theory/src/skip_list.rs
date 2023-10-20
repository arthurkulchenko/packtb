// Garbage implementation of skip lists

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

    // NOTICE: No propbalistic distribution across levels
    fn get_level() -> usize {
        if self.head.is_none() { return self.height + 1 }
        if self.height <= 0 { return 1; }

        let mut n = 0;
        while rand::random::<bool>() {
            n += 1;
        }
        n + 1
    }

    // FIXME: WIll constantly owerride tails earasing previous values
    fn update_tails(&mut self, length: usize, new_node: Node) {
        for index in 0..length {
            if let Some(previous) = self.tails[index].take() {
                let previous_next = &mut previous.borrow_mut().next();
                previous_next[index] = Some(new_node.clone());
            }
            self.tails[index] = Some(new_node.clone());
        }
    }


    pub fn insert(&mut self, value: String, offset: u64) {
        let level = self.get_level();
        let new_node = Node { value: value, next: vec![None; level], offset: offset };

        self.update_tails(level, new_node);

        if self.head.is_none() {
            self.head = Some(new_node.clone());
        }
        self.length += 1;
    }

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
