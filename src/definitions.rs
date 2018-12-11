use read_input::*;
use ansi_term::Colour::*;
use ansi_term::Style;

use std::time::Duration;

pub fn wait(time: Duration) {
    std::thread::sleep(time);
}

static WPM: f64 = 200.;
static WPS: f64 = WPM / 60.;
pub fn seconds_to_read(text: &str) -> f64 {
    text.split_whitespace().count() as f64 / WPS
}

pub fn narrate(text: &str) {
    println!("{}", Blue.paint(text));
    wait(Duration::from_float_secs(seconds_to_read(text)));
}

pub fn dialog(name: &str, text: &str) {
    println!("{}: {}", Red.paint(name), text);
    wait(Duration::from_float_secs(seconds_to_read(text)));
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

pub enum Race {
    HighElf, Argonian, WoodElf, Breton, DarkElf, Imperial, Khajit, Nord, Orc, Redguard,
}

pub enum Gender {
    Male,
    Female,
}

impl Gender {
    pub fn he_she(&self, capitalized: bool) -> &'static str {
        match self {
            Gender::Male => if capitalized { "He" } else { "he" },
            Gender::Female => if capitalized { "She" } else { "she" },
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

pub trait Item {
    fn name(&self) -> &str;
    fn weight(&self) -> f32;
    fn value(&self) -> u16;
}

pub struct Weapon {
    name: &'static str,
    base_damage: u16,
    weight: f32,
    value: u16,
}


impl Item for Weapon {
    fn name(&self) -> &str {
        &self.name
    }

    fn weight(&self) -> f32 {
        self.weight
    }

    fn value(&self) -> u16 {
        self.value
    }
}

pub struct Container {
    name: String,
    items: Vec<&'static Item>,
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
    pub fn new(name: &str, description: &str, items: Option<Vec<&'static Item>>, containers: Option<Vec<Container>>) -> Room {
        Room {
            name: name.to_owned(),
            description: description.to_owned(),
            items: match items { Some(t) => t, _ => Vec::new() },
            containers: match containers { Some(c) => c, _ => Vec::new() }
        }
    }
}

pub fn process_command(command: &str, current_room: &Room) {
    let cmd = command.to_lowercase();
    if cmd.contains("help") {
        println!("Items: {}", list_options(&current_room.items.iter().map(|item| item.name()).collect::<Vec<&str>>()));
        println!("Containers: {}", list_options(&current_room.containers.iter().map(|container| &container.name[..]).collect::<Vec<&str>>()));
    } else if cmd.contains("look") {
        narrate(&current_room.description);
    } else {
        println!("Unrecognized command.");
    }
}

pub static IMPERIAL_SWORD: Weapon = Weapon { name: "Imperial Sword", base_damage: 8, weight: 10., value: 23 };
