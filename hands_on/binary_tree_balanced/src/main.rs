use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T> {
    // parent
    data: T,
    height: u8,
    left: BTree<T>,
    right: BTree<T>,
}

impl<T> Node<T> {
    pub fn update_height(&mut self) {
        match (&self.left.0, &self.right.0) {
            (Some(left), Some(right)) => {
                if left.height > right.height {
                    self.height = left.height + 1;
                } else {
                    self.height = right.height + 1;
                }
            },
            (Some(left), None) => { self.height = left.height + 1; },
            (None, Some(right)) => { self.height = right.height + 1; },
            (None, None) => { self.height = 0; }
        }
    }

    pub fn rotate_left(mut self) -> Box<Self> {
        let mut result = match self.right.0.take() {
            Some(result) => result,
            None => return Box::new(self)
        };

        self.right = BTree(result.left.0.take());
        self.right.update_height();
        result.left = BTree(Some(Box::new(self)));
        result.left.update_height();
        result.height = 1 + std::cmp::max(result.left.height(), result.right.height());
        result
    }

    pub fn rotate_right(mut self) -> Box<Self> {
        let mut result = match self.left.0.take() {
            Some(result) => result,
            None => return Box::new(self)
        };

        self.left = BTree(result.left.0.take());
        self.left.update_height();
        result.right = BTree(Some(Box::new(self)));
        result.right.update_height();
        result.height = 1 + std::cmp::max(result.right.height(), result.left.height());
        result
    }
}

#[derive(Debug)]
pub struct BTree<T>(Option<Box<Node<T>>>);

impl<T> BTree<T> {
    pub fn new() -> Self {
        BTree(None)
    }

    pub fn height(&self) -> u8 {
        match self.0 {
            Some(ref node) => node.height,
            None => 0
        }
    }

    pub fn update_height(&mut self) {
        if let Some(ref mut node) = self.0 {
            node.height = 1 + std::cmp::max(node.left.height(), node.right.height())
        }
    }

    pub fn rotate_left(&mut self) {
        self.0 = self.0.take().map(|v| v.rotate_left());
    }

    pub fn rotate_right(&mut self) {
        self.0 = self.0.take().map(|v| v.rotate_right());
    }
}

// NOTICE: Primitive
impl<T: PartialOrd> BTree<T> {
    pub fn add(&mut self, data: T) {
        let rotation_direction = match self.0 {
            Some(ref mut branch_data) => {
                let direction = if data < branch_data.data {
                    branch_data.left.add(data);
                    if branch_data.left.height() - branch_data.right.height() > 1 {
                        // self.rotate_right()
                        1
                    } else {
                        0
                    }
                } else {
                    branch_data.right.add(data);
                    if branch_data.right.height() - branch_data.left.height() > 1 {
                        // self.rotate_left()
                        -1
                    } else {
                        0
                    }
                };
                branch_data.update_height();
                direction
            },
            None => {
                self.0 = Some(Box::new(Node { data: data, height: 0, left: BTree(None), right: BTree(None) }));
                0
            }
        };
        match rotation_direction {
            1 =>  self.rotate_right(),
            -1 =>  self.rotate_left(),
            _ => self.update_height(),
        };
        // self.update_height();
    }
}

impl <T: Debug> BTree<T> {
    pub fn print_lfirst(&self, depth: u32) {
        if let Some(ref node) = self.0 {
            node.left.print_lfirst(depth + 1);
            let mut spacing = String::new();
            for _ in 0..depth {
                spacing.push('.');
            }
            println!("{}{:?}", spacing, node.data);
            node.right.print_lfirst(depth + 1);
        }
    }
}

fn main() {
    let mut tree = BTree::new();
    tree.add(5);
    tree.add(4);
    tree.add(8);
    tree.add(5);
    tree.add(2);
    tree.add(1);
    // println!("{:?}", tree);
    tree.print_lfirst(0);
    tree.rotate_left();
    println!("=================");
    tree.print_lfirst(0);
}
