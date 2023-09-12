use std::fmt;
use core::fmt::{Debug,Write,Display};
use std::rc::Rc;
use std::cell::RefCell;
use rand;

type RRc<T> = Rc<RefCell<T>>;

pub fn rrc<T>(data: T) -> RRc<T> {
    Rc::new(RefCell::new(data))
}

#[derive(Debug)]
pub struct SkipList<T: PartialOrd>(Vec<SkipNode<T>>);

impl <T: PartialOrd + Clone> SkipList<T> {
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    pub fn insert(&mut self, data: T) {
        if self.0.len() == 0 {
            self.0.push(SkipNode::new(data));
            return;
        }
        // Our vec will have the lowest row, with the lowest number, we need to try and insert in the highest available row.
        for i in (0..self.0.len()).rev() {
            if data > *self.0[i].data.borrow() {
                if let Some(ch) = self.0[i].insert(data) {
                    // TODO loop up on 50/50 chance
                    self.loop_up(ch, i + 1);
                }
                return;
            }
            // if none of those succeeded/ that means we have an element to replace the first
            let mut nn = SkipNode::new(data.clone());
            std::mem::swap(&mut nn, &mut self.0[0]); // put out new element on the front of the row
            let res = rrc(nn);
            self.0[0].right = Some(res.clone());
            self.loop_up(res, 1);
        }
    }

    pub fn loop_up(&mut self, ch: RRc<SkipNode<T>>, n: usize) {
        if rand::random::<bool>() == true {
            return;
        }
        let dt = ch.borrow().data.clone();
        let mut nn = SkipNode {
            right: None,
            down: Some(ch),
            data: dt
        };
        if n >= self.0.len() {
            self.0.push(nn);
            return;
        }
        std::mem::swap(&mut nn, &mut self.0[n]);
        let res = rrc(nn);
        self.0[n].right = Some(res.clone());
        self.loop_up(res, n + 1);
    }
}

#[derive(Debug)]
pub struct SkipNode<T: PartialOrd> {
    right: Option<RRc<SkipNode<T>>>,
    down: Option<RRc<SkipNode<T>>>,
    data: RRc<T>
}

impl<T: PartialOrd> SkipNode<T> {
    pub fn new(data: T) -> Self {
        SkipNode { right: None, down: None, data: rrc(data) }
    }

    // so far we never make an up node, so all we really have is a linked list
    pub fn insert(&mut self, dt: T) -> Option<RRc<SkipNode<T>>> {
        // bigger then el to the right
        if let Some(ref mut rt) = self.right {
            // Why we need dereference to compare but do not need deref to mutate
            if dt > *rt.borrow().data.borrow() {
                return rt.borrow_mut().insert(dt);
            }
        }
        // has lower children
        if let Some(ref dw) = self.down {
            return match dw.borrow_mut().insert(dt) {
                Some(child) => match rand::random::<bool>() {
                    true => {
                        let dt = child.borrow().data.clone();
                        let nn = SkipNode { right: self.right.take(), down: Some(child), data: dt };
                        let result = rrc(nn);
                        self.right = Some(result.clone());
                        Some(result)
                    },
                    false => None
                },
                None => None
            }
        }
        // should be before right, at bottom node
        let mut nn = SkipNode::new(dt);
        nn.right = self.right.take();
        let result = rrc(nn);
        self.right = Some(result.clone());
        Some(result)
    }
}

impl<T: PartialOrd + Debug> SkipNode<T> {
    pub fn print_row<W: Write>(&self, w: &mut W) -> std::fmt::Result {
        write!(w, "{:?}", self.data.borrow())?;
        if let Some(ref r) = self.right {
            write!(w, ",")?;
            r.borrow_mut().print_row(w)
        } else {
            Ok(())
        }
    }
}

impl<T: PartialOrd + Debug> Display for SkipList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.len() == 0 {
            return write!(f, "SkipList<Empty>");
        }

        for sn in &self.0 {
            write!(f, "\n")?;
            sn.print_row(f)?
        }
        Ok(())
    }
}

fn main() {
    let mut snode = SkipList::new();
    snode.insert(4);
    snode.insert(5);
    snode.insert(33);
    snode.insert(39);
    snode.insert(19);
    println!("skip list{}", snode);
}
