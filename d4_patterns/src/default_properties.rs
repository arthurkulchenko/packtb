use std::collections::BTreeMap;
use d4_builder_derive::Setter;

// #[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
pub enum Trigger {
    BattleCry,
    Death,
    EnemyDeath,
    Damage,
}

pub trait Triggerable {
    fn trigger(&self, t: &Trigger) -> Option<String>;
}

pub struct TriggeerableWrap<A: Triggerable, B: Triggerable> {
    a: A,
    b: B,
}

impl <A: Triggerable, B: Triggerable> Triggerable for TriggeerableWrap<A, B> {
    fn trigger(&self, t: &Trigger) -> Option<String> {
        self.a.trigger(t).or_else(|| self.b.trigger(t))
    }
}

#[derive(Debug, PartialEq)]
pub enum Ability {
    Charge,
    Taunt
}

#[derive(Debug, PartialEq, Setter)]
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

impl Triggerable for Card {
    fn trigger(&self, t: &Trigger) -> Option<String> {
        self.triggers.get(t).map(|s| s.to_string())
    }
}

impl Default for Card {
    fn default() -> Self {
        Card {
            name: String::new(),
            strenght: 1,
            health: 1,
            cost: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        }
    }
}

#[derive(Debug, PartialEq, Default)]
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
            ..Default::default()
            // strenght: None,
            // health: None,
            // cost: None,
            // abilities: Vec::new(),
            // triggers: BTreeMap::new(),
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

    #[test]
    fn cards_builder_method_creates_card() {
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

    #[test]
    fn cards_have_default_trait() {
        let c = Card { name: "some name".to_string(), ..Default::default() };
        let c2 = Card {
            name: "some name".to_string(),
            strenght: 1,
            cost: 1,
            health: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        };
        assert_eq!(c, c2);
    }

    #[test]
    fn cards_have_default_impl() {
        let c = Card::default();
        let c2 = Card {
            name: "".to_string(),
            strenght: 1,
            cost: 1,
            health: 1,
            abilities: Vec::new(),
            triggers: BTreeMap::new(),
        };
        assert_eq!(c, c2);
    }

    #[test]
    fn trigger_wrap_helps_to_trigger_corresponsing_trigger() {
        let c = Card::build("c".to_string()).trigger(Trigger::BattleCry, "Cry me a river".to_string()).build();
        let c2 = Card::build("c2".to_string()).trigger(Trigger::Death, "You DIE".to_string()).build();
        let wrap = TriggeerableWrap { a: c, b: c2 };
        assert_eq!(wrap.trigger(&Trigger::BattleCry).unwrap(), "Cry me a river");
        assert_eq!(wrap.trigger(&Trigger::Death).unwrap(), "You DIE");
    }

    #[test]
    fn wip_macro_setter_derive() {
        let c = Card::build("c".to_string()).strenght(4).build();
        c.print();
        assert_eq!(c.strenght, 4);
    }
}
