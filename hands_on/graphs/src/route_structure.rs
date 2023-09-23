use std::collections::{HashMap, HashSet};
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

type RRoute<ID> = Rc<Route<ID>>;

#[derive(Debug)]
pub struct Route<ID> {
    position: ID,
    path: Option<Rc<Route<ID>>>,
    length: i32
}

impl <ID: Eq> Route<ID> {
    pub fn start_rc(position: ID) -> Rc<Self> {
        Rc::new(Route { position, path: None, length: 0 })
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
            return write!(f, "{}--{}", path, self.length);
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

impl <T, E: Weighted, ID: Clone + Hash + Eq> Graph<T,E,ID> {
    pub fn shortest_path(&self, from: ID, to: ID) -> Option<RRoute<ID>> {
        let mut visited: HashSet<ID> = HashSet::new();
        let mut routes = Vec::new();
        routes.push(Route::start_rc(from));

        loop {
            let current_route = routes.pop()?;
            if to == current_route.position { return Some(current_route); }
            if visited.contains(&current_route.position) { continue; }

            visited.insert(current_route.position.clone());
            let exist = self.data.get(&current_route.position)?;

            for existed_id in &exist.1 {
                let edge = self.edges.get(existed_id)?;
                // WHAT: What is npos
                let npos = if edge.1 == current_route.position {
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = current_route.length + edge.0.weight();
                let nroute = Rc::new(Route { position: npos, length: nlen, path: Some(current_route.clone()) });

                if routes.len() == 0 {
                    routes.push(nroute.clone());
                    continue;
                }

                let mut index_after = routes.len() - 1;
                loop {
                    if routes[index_after].length > nlen {
                        routes.insert(index_after + 1, nroute);
                        break;
                    }
                    if index_after == 0 {
                        routes.insert(0, nroute);
                        break;
                    }
                    index_after -= 1;
                }
            }
        }
    }

    pub fn shortest_path_r(&self, from: RRoute<ID>, to: ID) -> Option<RRoute<ID>> {
        let mut toset = HashSet::new();
        toset.insert(to);
        self.closest_path(from, &toset)
    }

    pub fn closest_path(&self, from: RRoute<ID>, to: &HashSet<ID>) -> Option<RRoute<ID>> {
        let mut visited: HashSet<ID> = HashSet::new();
        let mut routes = Vec::new();
        routes.push(from);

        loop {
            let current_route = routes.pop()?;
            if to.contains(&current_route.position) { return Some(current_route); }
            if visited.contains(&current_route.position) { continue; }

            visited.insert(current_route.position.clone());
            let exist = self.data.get(&current_route.position)?;

            for existed_id in &exist.1 {
                let edge = self.edges.get(existed_id)?;
                // WHAT: What is npos
                let npos = if edge.1 == current_route.position {
                    edge.2.clone()
                } else {
                    edge.1.clone()
                };
                let nlen = current_route.length + edge.0.weight();
                let nroute = Rc::new(Route { position: npos, length: nlen, path: Some(current_route.clone()) });

                if routes.len() == 0 {
                    routes.push(nroute.clone());
                    continue;
                }

                let mut index_after = routes.len() - 1;
                loop {
                    if routes[index_after].length > nlen {
                        routes.insert(index_after + 1, nroute);
                        break;
                    }
                    if index_after == 0 {
                        routes.insert(0, nroute);
                        break;
                    }
                    index_after -= 1;
                }
            }
        }
    }

    fn greedy_salesman(&self, start: ID) -> Option<RRoute<ID>> {
        let mut to_visit: HashSet<ID> = self.data.keys().map(|k| k.clone()).collect();
        to_visit.remove(&start);
        let mut route = Route::start_rc(start.clone());
        while to_visit.len() > 0 {
            route = self.closest_path(route, &to_visit)?;
            to_visit.remove(&route.position);
        }
        self.shortest_path_r(route, start)
    }
}

fn main() -> Result<(), GraphErr> {
    let mut graph = Graph::new();
    for x in vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'F', 'H'] {
        graph.add_node(x, ());
    }
    let _ = graph.add_edge('a','H', 'D', 4)?;
    let _ = graph.add_edge('b','D', 'C', 14)?;
    let _ = graph.add_edge('c','C', 'B', 2)?;
    let _ = graph.add_edge('d','H', 'A', 12)?;
    let _ = graph.add_edge('e','A', 'C', 22)?;
    let _ = graph.add_edge('f','H', 'G', 5)?;
    let _ = graph.add_edge('g','G', 'A', 13)?;
    let _ = graph.add_edge('h','A', 'F', 8)?;
    let _ = graph.add_edge('i','F', 'E', 7)?;
    let _ = graph.add_edge('j','C', 'F', 9)?;

    let route = Route::start_rc('A');

    println!("{}", route);
    println!("shortest_path A-D is {}", graph.shortest_path('A', 'D').unwrap());
    println!("shortest_path H-B is {}", graph.shortest_path('H', 'B').unwrap());
    println!("greedy salesman start from A = {}", graph.greedy_salesman('A').unwrap());

    Ok(())
}
