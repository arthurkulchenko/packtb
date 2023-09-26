use crate::gen::GenData;

pub trait EcsStore<T> {
    fn get(&self, g: GenData) -> Option<&T>;
    fn get_mut(&mut self, g: GenData) -> Option<&mut T>;
    fn insert(&mut self, g: GenData, t: T);
    fn remove(&mut self, g: GenData);

    fn for_each<F: FnMut(GenData, &T)>(&self, f: F);
    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, f: F);
}

pub struct VecStore<T> {
    items: Vec<Option<(u64, T)>>,
}

impl<T> VecStore<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }
}

impl<T> EcsStore<T> for VecStore<T> {
    fn insert(&mut self, g: GenData, t: T) {
        while g.position >= self.items.len() {
            self.items.push(None);
        }
        self.items[g.position] = Some((g.gen, t));
    }

    fn get(&self, g: GenData) -> Option<&T> {
        if let Some(Some((ig, d))) = self.items.get(g.position) {
            if *ig == g.gen { return Some(d) }
        }
        None
    }

    fn remove(&mut self, g: GenData) {
        if let Some(Some((ig,_))) = self.items.get(g.position) {
            if *ig == g.gen {
                self.items[g.position] = None;
            }
        }
    }

    // WHY: While trait obligates us follow the signature, we can use mutable variant of a function?
    fn for_each<F: FnMut(GenData, &T)>(&self, mut f: F) {
        for (index, item) in self.items.iter().enumerate() {
            if let Some((g, d)) = item {
              f(GenData { gen: *g, position: index }, d)
            }
        }
    }

    fn for_each_mut<F: FnMut(GenData, &mut T)>(&mut self, mut f: F) {
        for (index, item) in self.items.iter_mut().enumerate() {
            if let Some((g, d)) = item {
              f(GenData { gen: *g, position: index }, d)
            }
        }
    }

    fn get_mut(&mut self, g: GenData) -> Option<&mut T> { unimplemented!() }
}
