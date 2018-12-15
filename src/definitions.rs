use ansi_term::Colour::*;
use ansi_term::Style;
use read_input::*;

use std::time::Duration;

pub fn wait(time: Duration) {
    if cfg!(debug_assertions) == false {
        std::thread::sleep(time);
    }
}

static WPM: f64 = 200.;
static WPS: f64 = WPM / 60.;
pub fn seconds_to_read(text: &str) -> f64 {
    text.split_whitespace().count() as f64 / WPS
}

pub fn narrate(text: &str) {
    println!("{}", Blue.paint(text));
    wait(Duration::from_float_secs(seconds_to_read(text)) + Duration::from_secs(1));
}

pub fn dialog(name: &str, text: &str) {
    println!("{}: {}", Red.paint(name), text);
    wait(Duration::from_float_secs(seconds_to_read(text)) + Duration::from_secs(1));
}

pub fn list_options(options: &[&str]) -> String {
    assert!(options.len() > 0);
    let mut commas = options.len() - 1;
    let mut output = String::new();
    for item in options {
        output.push_str(&Green.paint(*item).to_string());
        if commas != 0 {
            output.push_str(", ");
            commas -= 1;
        }
    }
    output
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ItemType<'a> {
    Weapon(&'a Weapon),
    Clothing(&'a Clothing),
    Armor(&'a Armor),
}

pub trait Item {
    fn name(&self) -> &str;
    fn weight(&self) -> u16;
    fn value(&self) -> u16;
    fn intrinsic(&self) -> ItemType;
}

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
        }
    }

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
pub struct Clothing {
    name: &'static str,
    weight: u16,
    value: u16,
}

impl Item for Clothing {
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
        ItemType::Clothing(&self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Armor {
    name: &'static str,
    base_armor: u16,
    weight: u16,
    value: u16,
}

impl Armor {
    pub fn armor(&self) -> u16 {
        self.base_armor
    }
}

impl Item for Armor {
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
        ItemType::Armor(&self)
    }
}

pub struct Container {
    name: String,
    pub items: Vec<&'static Item>,
}

impl Container {
    pub fn new(name: &str, items: Vec<&'static Item>) -> Container {
        Container {
            name: name.to_owned(),
            items,
        }
    }
}

pub struct Room {
    name: String,
    description: String,
    items: Vec<&'static Item>,
    containers: Vec<Container>,
    // people
}

impl Room {
    pub fn new(
        name: &str,
        description: &str,
        items: Option<Vec<&'static Item>>,
        containers: Option<Vec<Container>>,
    ) -> Room {
        Room {
            name: name.to_owned(),
            description: description.to_owned(),
            items: match items {
                Some(t) => t,
                _ => Vec::new(),
            },
            containers: match containers {
                Some(c) => c,
                _ => Vec::new(),
            },
        }
    }
}

pub fn split_whitespace_with_quotes(text: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if !c.is_whitespace() {
            match c {
                '"' => {
                    let mut string = String::new();
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        } else {
                            string.push(c);
                        }
                    }

                    items.push(string);
                }
                _ => {
                    let mut word = c.to_string();
                    while let Some(&pc) = chars.peek() {
                        if pc == '"' || pc.is_whitespace() {
                            break;
                        } else {
                            word.push(chars.next().unwrap());
                        }
                    }

                    items.push(word);
                }
            }
        }
    }

    items
}

pub fn process_command(command: &str, player: &mut Player, current_room: &mut Room) {
    let cmd = command.to_lowercase();
    if cmd.contains("help") {
        println!("Commands: {}", list_options(&["look", "inventory [container]", "take <items>", "ctake <container> <items>"]));
    } else if cmd.starts_with("look") {
        println!("{}", &current_room.description);
        if current_room.items.len() > 0 {
            println!(
                "Items: {}",
                list_options(
                    &current_room
                        .items
                        .iter()
                        .map(|item| item.name())
                        .collect::<Vec<&str>>()
                )
            );
        }
        if current_room.containers.len() > 0 {
            println!(
                "Containers: {}",
                list_options(
                    &current_room
                        .containers
                        .iter()
                        .map(|container| &container.name[..])
                        .collect::<Vec<&str>>()
                )
            );
        }
    } else if cmd.starts_with("inventory") {
        let words: Vec<&str> = cmd.split_whitespace().collect();
        let mut inventory: &Container = &player.inventory;
        
        if words.len() > 1 {
            let container_name = words[1].to_lowercase();
            let mut found_container = false;

            for container in &current_room.containers {
                if container.name.to_lowercase() == container_name {
                    inventory = container;
                    found_container = true;
                }
            }

            if !found_container {
                println!("Usage: `inventory [container]` where you can optionally list a container to see inside of.");
                return;
            }
        } else if words.len() == 1 {
            inventory = &player.inventory;
        } else {
            println!("Usage: `inventory [container]` where you can optionally list a container to see inside of.");
            return;
        }

        if inventory.items.is_empty() {
            println!("No items in {}", &inventory.name);
        } else {
            println!(
                "Items: {}",
                list_options(
                    &inventory
                        .items
                        .iter()
                        .map(|item| item.name())
                        .collect::<Vec<&str>>()
                )
            );
        }
    } else if cmd.starts_with("ctake") {
        let words = split_whitespace_with_quotes(&cmd);
        if words.len() < 3 {
            println!("{}", words.len());
            println!("Usage: `ctake <container> <items>` where items can be one or a list of item names to take from the given container.");
        } else {
            let mut words = words.iter();
            words.next().unwrap(); // consume "ctake"
            let container_name = words.next().unwrap();

            let mut inventory: &Container = &player.inventory;
            let mut found_container = false;

            for container in &current_room.containers {
                if &container.name.to_lowercase() == container_name {
                    inventory = container;
                    found_container = true;
                }
            }

            if !found_container {
                println!("Could not find given container\nUsage: `ctake <container> <items>` where items can be one or a list of item names to take from the given container.");
                return;
            }

            let mut item_names = Vec::new();
            for name in words {
                item_names.push(name);
            }

            let mut items = Vec::<&Item>::new();
            for item in &item_names {
                for &citem in &inventory.items {
                    if item == &&citem.name().to_lowercase() {
                        items.push(citem);
                        // remove items in room
                    }
                }
            }

            player.inventory.items.extend(&items);
        }
    } else if cmd.starts_with("take") {
        let item_names: Vec<&str> = cmd.split_whitespace().collect();
        if item_names.len() > 1 {
            let mut items = Vec::<&Item>::new();
            for item in &item_names[1..] {
                for &room_item in &current_room.items {
                    if item == &room_item.name() {
                        items.push(room_item);
                        // remove items in room
                    }
                }
            }

            player.inventory.items.extend(&items);
            for item in &items {
                // or remove items from room here
            }
        } else {
            println!("Usage: `take <items>` where `items` is a list of items in the room to pickup.");
        }
    } else {
        println!("Unrecognized command. Try 'help' for a list of commands.");
    }
}

pub static IMPERIAL_SWORD: Weapon = Weapon {
    name: "Imperial Sword",
    base_damage: 8,
    weight: 10,
    value: 23,
};

pub static FOOTWRAPS: Clothing = Clothing {
    name: "Footwraps",
    weight: 1,
    value: 1,
};

pub static ROUGHSPUN_TUNIC: Clothing = Clothing {
    name: "Roughspun Tunic",
    weight: 1,
    value: 1,
};
