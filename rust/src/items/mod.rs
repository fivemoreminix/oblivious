pub mod apparel;

use std::fmt;
use crate::definitions::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ItemType<'a> {
    Weapon(&'a Weapon),
    Clothing(&'a Clothing),
    Armor(&'a Armor),
    Key(&'a Key),
    Potion(&'a Potion),
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

impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.name() == other.name()
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PotionType {
    Health,
    Magicka,
    Stamina,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Potion {
    name: &'static str,
    ptype: PotionType,
    strength: u16,
    value: u16,
}

impl Item for Potion {
    fn name(&self) -> &str {
        self.name
    }

    fn weight(&self) -> u16 {
        1
    }

    fn value(&self) -> u16 {
        self.value
    }

    fn intrinsic(&self) -> ItemType {
        ItemType::Potion(&self)
    }
}

pub static MINOR_HEALTH: Potion = Potion {
    name: "Potion of Minor Health",
    ptype: PotionType::Health,
    strength: 25,
    value: 17,
};

pub static MINOR_MAGICKA: Potion = Potion {
    name: "Potion of Minor Magicka",
    ptype: PotionType::Magicka,
    strength: 25,
    value: 20,
};

pub static MINOR_STAMINA: Potion = Potion {
    name: "Potion of Minor Stamina",
    ptype: PotionType::Stamina,
    strength: 25,
    value: 20,
};

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