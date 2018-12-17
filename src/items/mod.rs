pub mod apparel;

use std::fmt;
use crate::definitions::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ItemType<'a> {
    Weapon(&'a Weapon),
    Clothing(&'a Clothing),
    Armor(&'a Armor),
    Key(&'a Key),
}

pub trait Item {
    fn name(&self) -> &str;
    fn weight(&self) -> u16;
    fn value(&self) -> u16;
    fn intrinsic(&self) -> ItemType;
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.name())
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Weapon {
    name: &'static str,
    base_damage: u16,
    weight: u16,
    value: u16,
}

impl Item for Weapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn weight(&self) -> u16 {
        self.weight
    }

    fn value(&self) -> u16 {
        self.value
    }

    fn intrinsic(&self) -> ItemType {
        ItemType::Weapon(&self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Key {
    name: &'static str,
}

impl Item for Key {
    fn name(&self) -> &str {
        self.name
    }

    fn weight(&self) -> u16 {
        0
    }

    fn value(&self) -> u16 {
        0
    }

    fn intrinsic(&self) -> ItemType {
        ItemType::Key(&self)
    }
}

pub static IMPERIAL_SWORD: Weapon = Weapon {
    name: "Imperial Sword",
    base_damage: 8,
    weight: 10,
    value: 23,
};

pub static IRON_SWORD: Weapon = Weapon {
    name: "Iron Sword",
    base_damage: 7,
    weight: 9,
    value: 25,
};

pub static HELGEN_KEEP_KEY: Key = Key {
    name: "Helgen Keep Key",
};