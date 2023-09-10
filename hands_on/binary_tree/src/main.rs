use std::fmt::Debug;

#[derive(Debug)]
pub struct Node<T> {
    // parent
    data: T,
    left: BTree<T>,
    right: BTree<T>,
}

#[derive(Debug)]
pub struct BTree<T>(Option<Box<Node<T>>>);

impl<T> BTree<T> {
    pub fn new() -> Self {
        BTree(None)
    }
}

// NOTICE: Primitive
impl<T: PartialOrd> BTree<T> {
    pub fn add(&mut self, data: T) {
        match self.0 {
            Some(ref mut brach_data) => {
                if data < brach_data.data {
                    brach_data.left.add(data);
                } else {
                    brach_data.right.add(data);
                }
            },
            None => { self.0 = Some(Box::new(Node { data: data, left: BTree(None), right: BTree(None) })); }
        }
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
}
