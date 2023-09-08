#[derive(Debug, Copy)]
pub struct LinkedList<T>(
    Option<(T, Node<T>)>
);

type Node<T> = Box<LinkedList<T>>;

impl<T> LinkedList<T> {
    // Working with List holder
    pub fn new() -> Self {
        LinkedList(None)
    }

    pub fn push_front(&mut self, data: T) {
        let t = self.0.take(); // Option(Tuple) = LiknedList
        self.0 = Some(
            (data, Box::new(LinkedList(t)))
        );
    }

    pub fn push_back(&mut self, data: T) {
        let mut list = self;
        while let Some((_data, nested_node)) = &mut list.0 {
            list = *nested_node;
        }
        list.0 = Some((data, Box::new(LinkedList(None))));
    }
}
fn main() {
    println!("Hello, world!");
}
