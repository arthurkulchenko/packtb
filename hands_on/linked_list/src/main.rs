#[derive(Debug)]
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
        match self.0 {
            Some((_, ref mut node)) => node.push_back(data),
            None => self.push_front(data),
        }
    }
}
fn main() {
    let mut ll = LinkedList::new();
    ll.push_front(5);
    ll.push_back(6);
    ll.push_front(4);
    println!("{:?}", ll);
}
