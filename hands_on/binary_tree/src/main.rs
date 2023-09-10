type NodeRef<T> = Box<Node<T>>;

#[derive(Debug)]
pub struct BTree<T>(Option<NodeRef<T>>);

impl<T> BTree<T> {
    pub fn new() -> Self {
        BTree(None)
    }
}

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

#[derive(Debug)]
pub struct Node<T> {
    // parent
    data: T,
    left: BTree<T>,
    right: BTree<T>,
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
}
