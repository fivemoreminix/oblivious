use crate::definitions::*;

#[derive(Clone)]
pub struct Player {
    name: String,
    race: Race,
    gender: Gender,
    inventory: Container,
    max_health: u32,
    max_stamina: u32,
    max_magicka: u32,
    max_weight: u16,
    health: u32,
    stamina: u32,
    magicka: u32,
    weight: u16,
    pub apparel: ApparelPlacement,
}

impl Player {
    pub fn new(name: &str, race: Race, gender: Gender) -> Player {
        Player {
            name: name.to_owned(),
            race,
            gender,
            inventory: Container::new("Inventory", Vec::new()),
            max_health: 100,
            max_stamina: 100,
            max_magicka: 100,
            max_weight: 100,
            health: 100,
            stamina: 100,
            magicka: 100,
            weight: 100,
            apparel: ApparelPlacement::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn items(&self) -> &[&'static Item] {
        &self.inventory.items
    }

    /// Add a single item to the player's inventory.
    /// Returns true if the item was accepted. False if
    /// the item could not be added to the inventory.
    pub fn add_item(&mut self, item: &'static Item) -> bool {
        if self.weight + item.weight() <= self.max_weight {
            self.inventory.items.push(item);
            true
        } else {
            false
        }
    }

    pub fn add_items(&mut self, items: &[&'static Item]) -> Option<Vec<&'static Item>> {
        let mut rejected = Vec::<&'static Item>::new();
        for &item in items {
            if !self.add_item(item) {
                rejected.push(item);
            }
        }

        println!("Acquired {} item{}", items.len(), if items.len() > 1 { 's' } else { ' ' });

        if rejected.is_empty() {
            None
        } else {
            Some(rejected)
        }
    }

    /// Returns true if the item has been removed. False otherwise.
    pub fn remove_item(&mut self, item: &'static Item) -> bool {
        self.inventory.items.remove_item(&item).is_some()
    }

    pub fn remove_items(&mut self, items: &[&'static Item]) -> Option<Vec<&'static Item>> {
        let mut rejected = Vec::<&'static Item>::new();
        for &item in items {
            if !self.remove_item(item) {
                rejected.push(item);
            }
        }

        println!("Removed {} item{}", items.len(), if items.len() > 1 { 's' } else { ' ' });

        if rejected.is_empty() {
            None
        } else {
            Some(rejected)
        }
    }

    // pub fn inventory_apparel(&self) -> Vec<&impl Apparel> {
    //     self.inventory
    //         .items
    //         .iter()
    //         .filter_map(|i| {
    //             let ret: Option<&impl Apparel> = match i.intrinsic() {
    //                 ItemType::Clothing(v) | ItemType::Armor(v) => Some(v),
    //                 _ => None,
    //             }
    //             ret
    //         })
    //         .collect()
    // }

    pub fn inventory_weapons(&self) -> Vec<&Weapon> {
        self.inventory
            .items
            .iter()
            .filter_map(|i| match i.intrinsic() {
                ItemType::Weapon(weapon) => Some(weapon),
                _ => None,
            })
            .collect()
    }

    pub fn inventory_keys(&self) -> Vec<&Key> {
        self.inventory
            .items
            .iter()
            .filter_map(|i| match i.intrinsic() {
                ItemType::Key(key) => Some(key),
                _ => None,
            })
            .collect()
    }

    pub fn inventory_potions(&self) -> Vec<&Potion> {
        self.inventory
            .items
            .iter()
            .filter_map(|i| match i.intrinsic() {
                ItemType::Potion(potion) => Some(potion),
                _ => None,
            })
            .collect()
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Race {
    HighElf,
    Argonian,
    WoodElf,
    Breton,
    DarkElf,
    Imperial,
    Khajit,
    Nord,
    Orc,
    Redguard,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn he_she(&self, capitalized: bool) -> &'static str {
        match self {
            Gender::Male => {
                if capitalized {
                    "He"
                } else {
                    "he"
                }
            }
            Gender::Female => {
                if capitalized {
                    "She"
                } else {
                    "she"
                }
            }
        }
    }

    pub fn his_her(&self) -> &'static str {
        match self {
            Gender::Male => "his",
            Gender::Female => "her",
        }
    }

    pub fn boy_girl(&self) -> &'static str {
        match self {
            Gender::Male => "boy",
            Gender::Female => "girl",
        }
    }
}
