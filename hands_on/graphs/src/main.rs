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



fn main() {
    println!("Hello, world!");
}
