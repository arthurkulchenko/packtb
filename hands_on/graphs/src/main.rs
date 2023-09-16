// use std::rc::{Rc, Weak};
// use std::cell::RefCell;
use std::collections::HashMap;
use std::hash::Hash;

// ============================================================================

// type RRc<T> = Rc<RefCell<T>>;

// pub fn rrc<T>(data: T) -> RRc<T> {
//     Rc::new(RefCell::new(data))
// }

// pub struct EdgeListGraph<E, ID> {
//     // Data on the edges at E dont care too much about the nodes.
//     // Simpl, but can be slow at traversal
//     v: Vec<(E, ID, ID)>,
// }

// // Pointer based good for directed graphs as edges can go one way.
// // Using Weak pointers means the edge will fail safely if node has been removed
// // can stick Edge data if needed
// pub struct RRcGraph<T, E> {
//     nodes: Vec<RRc<RRcGraph<T, E>>>
// }

// pub struct RRcNode<T, E> {
//     data: T,
//     edges: Vec<(E, Weak<RefCell<RRcNode<T, E>>>)>
// }

// ============================================================================

// pub struct MapGraph<T, E, ID: Hash + Eq> {
//     mp: HashMap<ID, T>,
//     edges: Vec<(E, ID, ID)>
// }

// ============================================================================

// pub struct MapPGraph<T,E, ID: Hash + Eq> {
//     data: HashMap<ID, (T, Vec<ID>)>,
//     edges: HashMap<ID, (E, ID, ID)>
// }

// ============================================================================

#[derive(Debug)]
pub struct GraphErr {
    message: String,
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr { message: s.to_string() }
    }
}

#[derive(Debug)]
pub struct Graph<T,E, ID: Hash + Eq> {
    data: HashMap<ID, (T, Vec<ID>)>,
    edges: HashMap<ID, (E, ID, ID)>
}

impl<T, E, ID: Clone + Hash + Eq> Graph<T, E, ID> {
    pub fn new() -> Self {
        Graph { data: HashMap::new(), edges: HashMap::new() }
    }

    pub fn add_node(&mut self, id: ID, data: T) {
        self.data.insert(id, (data, Vec::new()));
    }

    pub fn add_edge(&mut self, ed_id: ID, from: ID, to: ID, edat: E) -> Result<(), GraphErr> {
        if !self.data.contains_key(&from) {
            return Err(GraphErr::new("'from' not in nodes"));
        }

        if let Some(ref mut data) = self.data.get_mut(&to) {
            self.edges.insert(ed_id.clone(), (edat, from.clone(), to));
            data.1.push(ed_id.clone());
        } else {
            return Err(GraphErr::new("'to' not in nodes"));
        }

        self.data.get_mut(&from).unwrap().1.push(ed_id);
        Ok(())
    }
}

fn main() -> Result<(), GraphErr> {
    let mut graph = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'F', 'H'] {
        graph.add_node(x, ());
    }
    let _ = graph.add_edge('a','H', 'D', 4)?;
    let _ = graph.add_edge('b','C', 'A', 14)?;
    let _ = graph.add_edge('c','D', 'F', 2)?;
    let _ = graph.add_edge('d','H', 'A', 12)?;
    let _ = graph.add_edge('e','E', 'D', 9)?;
    let _ = graph.add_edge('f','H', 'F', 3)?;
    let _ = graph.add_edge('g','G', 'B', 7)?;
    let _ = graph.add_edge('f','C', 'D', 22)?;
    let _ = graph.add_edge('h','H', 'C', 17)?;
    println!("Hello, graph world!\n{:?}", graph);
    Ok(())
}
