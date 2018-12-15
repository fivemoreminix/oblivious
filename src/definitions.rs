use ansi_term::Colour::*;

use std::time::Duration;

pub use crate::items::*;
pub use crate::items::apparel::*;
pub use crate::game::*;
pub use crate::player::*;

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

#[derive(Clone)]
pub struct Container {
    pub name: String,
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

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub description: String,
    pub items: Vec<&'static Item>,
    pub containers: Vec<Container>,
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
