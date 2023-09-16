use std::collections::HashMap;
use std::hash::Hash;
use std::rc::Rc;
use std::fmt;

#[derive(Debug)]
pub struct GraphErr {
    message: String
}

impl GraphErr {
    pub fn new(s: &str) -> Self {
        GraphErr { message: s.to_string() }
    }
}

pub trait Weighted {
    fn weight(&self) -> i32;
}

impl Weighted for i32 {
    fn weight(&self) -> i32 {
        *self
    }
}

#[derive(Debug)]
pub struct Route<ID> {
    position: ID,
    path: Option<Rc<Route<ID>>>,
    lenght: i32
}

impl <ID: Eq> Route<ID> {
    pub fn start_rc(position: ID) -> Rc<Self> {
        Rc::new(Route { position, path: None, lenght: 0 })
    }

    pub fn contains(&self, id: &ID) -> bool {
        if self.position == *id {
            return true;
        }
        match self.path {
            Some(ref p) => p.contains(id),
            None => false
        }
    }
}

impl<ID: fmt::Debug> fmt::Display for Route<ID> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref path) = self.path {
            return write!(f, "{}--{}", path, self.lenght);
        }
        write!(f, "{:?}", self.position)
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

    let route = Route::start_rc('A');

    println!("{}", route);

    Ok(())
}
