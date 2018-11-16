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

    pub fn his_hers(&self) -> &'static str {
        match self {
            Gender::Male => "his",
            Gender::Female => "hers",
        }
    }

    pub fn boy_girl(&self) -> &'static str {
        match self {
            Gender::Male => "boy",
            Gender::Female => "girl",
        }
    }
}

trait Item {
    fn weight(&self) -> f32;
    fn value(&self) -> u16;
}

struct Weapon {
    name: String,

}


impl Item for Weapon {
    fn weight(&self) -> f32 {
        1.
    }
}

struct Container {
    name: String,
    items: Vec<&'static Item>,
    weight_cap: f32,
}

struct Room {
    name: String,
    description: String,
    items: Vec<&'static Item>,
    containers: Vec<Container>,
}
