use crate::definitions::*;

#[derive(Clone)]
pub struct Player {
    name: String,
    race: Race,
    gender: Gender,
    pub inventory: Container,
    max_health: u32,
    max_stamina: u32,
    max_magicka: u32,
    health: u32,
    stamina: u32,
    magicka: u32,
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
            health: 100,
            stamina: 100,
            magicka: 100,
            apparel: ApparelPlacement::new(),
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
