use std::rc::Rc;
use std::cell::RefCell;
use rand;

type RRc<T> = Rc<RefCell<T>>;

pub fn rrc<T>(data: T) -> RRc<T> {
    Rc::new(RefCell::new(data))
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

fn main() {
    let mut snode = SkipNode::new(9);
    snode.insert(4);
    snode.insert(5);
    snode.insert(33);
    snode.insert(39);
    snode.insert(19);
    println!("{:?}", snode);
}
