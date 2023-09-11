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
}

// NOTICE: Primitive
impl<T: PartialOrd> BTree<T> {
    pub fn add(&mut self, data: T) {
        match self.0 {
            Some(ref mut branch_data) => {
                if data < branch_data.data {
                    branch_data.left.add(data);
                } else {
                    branch_data.right.add(data);
                }
                branch_data.update_height();
            },
            None => { self.0 = Some(Box::new(Node { data: data, height: 0, left: BTree(None), right: BTree(None) })); }
        }
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
    println!("{:?}", tree);
    // tree.print_lfirst(0);
}
