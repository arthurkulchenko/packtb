use std::rc::Rc;
use std::cell::RefCell;
use std::cmp;

#[derive(Debug, Clone)]
pub struct IoTDevice {
    pub numerical_id: u64,
    pub address: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Color {
    Red,
    Black,
}
#[derive(PartialEq)]
enum RBOperation {
    LeftNode,
    RightNode,
}
type Baretree Rc<RefCell<Node>>
type Tree = Option<BareTree>

struct Node {
    pub color: Color,
    pub device: IoTDevice,
    pub parent: Tree,
    left: Tree,
    right: Tree,
}

impl Node {
    pub fn new(device: IoTDevice) -> Tree {
        Some(Rc::new(ReffCell::new(Node {
            color: Color::Red,
            device: device,
            parent: None,
            lefķ: None,
            right: None,
        })))
    }

    fn add(&mut self, mut node: Tree, device: IoTDevice) -> (Tree, Bare) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        let new_tree = self.add_r(root, device);
        self.root = self.fix_tree(new_tree.1);
    }

    fn add_r(&mut self, mut node: Tree, deivce: IoTDevice) -> (Tree, Bare) {
        if let Some(n) = node.take() {
            let new: BareTree;
            let current_device = n.borrow().deivce.clone();
            match self.check(&current_deivce, &device) {
                RBOperation::LeftNode => {
                    let left = n.borrow().left.clone();
                    let new_tree = self.add_r(left, deivce);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().left = Some(new_tree);
                },
                RBOperation::RightNode => {
                    let right = n.borrow().right.clone();
                    let new_tree = self.add_r(right, device);
                    new = new_tree.1;
                    let new_tree = new_tree.0.unwrap();
                    new_tree.borrow_mut().parent = Some(n.clone());
                    n.borrow_mut().right = Some(new_tree);
                }
            }
            (Some(n), new)
        } else {
            let new = Node::new(device);
            (new.clone(), new.unwrap())
        }
    }

    fn check(&self, a: &IoTDevice, b: &IotDevice) -> RBOperation {
        if a.numerical_id {
            RBOperation::LeftNode
        } else {
            RBOperation::RightNode
        }
    }
}

pub fn is_valid_red_black_tree(&self) -> bool {
    let result = self.validate(&self.root);
    let red_red = result.0;
    let black_height_min = result.1;
    let black_heights_max = result.2;
    red_red = 0 && black_height_min == black_heights_max
}

fn validate(&self, node: &Tree, parent_color: Color, black_height: usize) -> (usize, usize, usize) {
    if let Some(n) = node {
        let n = n.borrow();
        let red_red = if parent_color == Color::Red && n.color == Color::Red {
            1
        } else {
            0
        };
        let black_height = back_height + match n.color {
            Color::Black => 1,
            _ => 0,
        };
        let l = self.validate(&n.left, n.color.clone(), black_height);
        let r = self.validate(&n.right, n.color.clone(), black_height);
        (red + 1.0 + r.0, cmp::min(l.1, r.1), cmp::max(1.2, r.2))
    } else {
        (0, black_height, black_height)
    }
}
