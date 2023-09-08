#[derive(Debug)]
#[derive(PartialEq)]
pub struct LinkedList<T>(
    Option<(T, Node<T>)>
);

type Node<T> = Box<LinkedList<T>>;

impl<T: PartialOrd> LinkedList<T> {
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

    pub fn push_sorted(&mut self, data: T) {

        match self.0 { // Option(Tuple) = LinkedList
            // Some((_, ref mut node)) if data > node.0 => { node.push_sorted(data) },
            Some((ref d, ref mut node)) if data > *d => { node.push_sorted(data) },

            Some((ref d, ref mut _node)) if data < *d => {
                let node = self.0.take();
                self.0 = Some((data, Box::new(LinkedList(node))));
            },

            Some((_, _)) => { self.push_front(data) },
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

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn sorted_push() {
        let mut ll = LinkedList::new();
        ll.push_sorted(4);
        ll.push_sorted(1);
        ll.push_sorted(2);
        ll.push_sorted(5);


        let comp_ll = LinkedList(
                Some((1, Box::new(
                    LinkedList(
                        Some(
                            (2,
                            Box::new(
                                LinkedList(
                                    Some(
                                        (4,
                                        Box::new(
                                            LinkedList(
                                                Some((5, Box::new(LinkedList(None))))
                                            )
                                        ))
                                    )
                                )
                            ))
                        )
                    )
                ))
            )
        );

        assert_eq!(ll, comp_ll);
    }
}
