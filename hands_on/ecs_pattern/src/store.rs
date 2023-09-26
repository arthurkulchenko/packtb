#[derive(Copy, Clone, PartialEq, Debug)]
pub struct GenData {
    position: usize,
    gen: u64
}

pub struct EntityActive {
    active: bool,
    gen: u64
}

pub struct GenManager {
    items: Vec<EntityActive>,
    drops: Vec<usize>
}

impl GenManager {
    pub fn new() -> Self {
        Self { items: Vec::new(), drops: Vec::new() }
    }

    pub fn next(&mut self) -> GenData {
        if let Some(location) = self.drops.pop() {
            let active_entity = &mut self.items[location];
            active_entity.active = true;
            active_entity.gen += 1;
            return GenData { position: location, gen: active_entity.gen };
        }
        self.items.push(EntityActive { active: true, gen: 0 });
        return GenData { position: self.items.len() - 1, gen: 0 };
    }

    pub fn drop(&mut self, g: GenData) {
      if let Some(active_entity) = self.items.get_mut(g.position) {
          if active_entity.active && active_entity.gen == g.gen {
              active_entity.active = false;
              self.drops.push(g.position);
          }
      }
    }
}

#[cfg(test)]
mod specs {
    use super::*;

    #[test]
    fn items_drop() {
        let mut gm = GenManager::new();
        let g = gm.next();
        assert_eq!(g, GenData { position: 0, gen: 0 })
    }
}
