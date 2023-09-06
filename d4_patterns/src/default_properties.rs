use std::collections::BTreeMap;

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

#[derive(Debug, PartialEq)]
pub struct Card {
    pub name: String,
    pub strenght: i32,
    pub health: i32,
    pub cost: i32,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl Card {
    pub fn build(name: String) -> CardBuilder {
        CardBuilder::new(name)
    }
}

#[derive(Debug, PartialEq)]
pub struct CardBuilder {
    pub name: String,
    pub strenght: Option<i32>,
    pub health: Option<i32>,
    pub cost: Option<i32>,
    pub abilities: Vec<Ability>,
    pub triggers: BTreeMap<Trigger, String>,
}

impl CardBuilder {
    pub fn new(name: String) -> Self {
        CardBuilder {
            name,
            strenght: None,
            health: None,
            cost: None,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        }
    }

    pub fn strenght(mut self, s: i32) -> Self {
        self.strenght = Some(s);
        self
    }

    pub fn trigger(mut self, t: Trigger, s: String) -> Self {
        self.triggers.insert(t, s);
        self
    }

    pub fn build(self) -> Card {
        Card {
            name: self.name,
            strenght: self.strenght.unwrap_or(1),
            health: self.health.unwrap_or(1),
            cost: self.cost.unwrap_or(1),
            abilities: self.abilities,
            triggers: self.triggers,
        }
    }
}

#[cfg(test)]
mod specs {
    use super::*;
    use crate::{Card, Trigger};

    #[test]
    fn cards_get_builded() {
        let c = Card::build("General Blight".to_string()).strenght(4).trigger(Trigger::BattleCry, "Deal 2 Damage".to_string()).build();
        let mut c2_triggers = BTreeMap::new();
        c2_triggers.insert(Trigger::BattleCry, "Deal 2 Damage".to_string());
        let c2 = Card {
            name: "General Blight".to_string(),
            strenght: 4,
            cost: 1,
            health: 1,
            abilities: Vec::new(),
            triggers: c2_triggers,
        };
        assert_eq!(c, c2);
    }
}
