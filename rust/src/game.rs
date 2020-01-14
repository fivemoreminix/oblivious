use ansi_term::Colour::*;

use crate::definitions::*;

use std::collections::HashMap;

pub fn filter_text(text: &str) -> String {
    text.chars().filter(|&c| c != '\'').collect::<String>().to_lowercase()
}

pub fn split_whitespace_with_quotes(text: &str) -> Vec<String> {
    let mut items = Vec::new();
    let mut chars = text.chars().filter(|&c| c != '\'').peekable();

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

pub fn help(options: &[impl ToString]) {
    println!("Available commands: {}", list_options(options));
    println!("Use double quotes for multi-word command arguments. E.x. take \"iron sword\"");
    println!("For information on using a command, try the name of the command and a question mark immediately following: look?");
    println!("Usage: <> means one required, [] means zero or one required, {{}} means zero or more required");
}

pub fn look(command: &str, current_room: &mut Room) {
    if command.starts_with("look?") {
        println!("Displays information about the current room: its items and containers.\nUsage: look");
    } else {
        println!("{}: {}", Purple.paint(&current_room.name), Blue.paint(&current_room.description));
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
    }
}

pub fn inventory(command: &str, player: &mut Player, current_room: &mut Room) {
    if command.starts_with("inventory?") {
        println!("Prints the items of the given container. Without any given container name, this will display the items in your inventory.");
        println!("Usage: inventory [container]");
    } else {
        let words: Vec<String> = split_whitespace_with_quotes(&filter_text(command));
        let mut inventory: Option<&Container> = None;
        
        if words.len() > 1 {
            let container_name = words[1].to_lowercase();
            let mut found_container = false;

            for container in &current_room.containers {
                if filter_text(&container.name) == container_name {
                    inventory = Some(container);
                    found_container = true;
                }
            }

            if !found_container {
                println!("Could not find container {:?}", container_name);
                return;
            }
        } else if words.len() == 1 {
            inventory = None;
        } else {
            println!("Usage: `inventory [container]` where you can optionally list a container to see inside of.");
            return;
        }

        if match inventory { Some(c) => c.items.is_empty(), None => player.items().is_empty() } {
            println!("No items in {}", match inventory { Some(c) => &c.name, None => "inventory" });
        } else {
            let inventory = match inventory {
                Some(c) => c.items.as_slice(),
                None => player.items(),
            };

            let mut item_quantities = HashMap::new();
            for item in inventory.iter().map(|item| item.name()) {
                *item_quantities.entry(item).or_insert(0) += 1;
            }

            let mut output = String::new();
            let mut commas = if item_quantities.len() > 1 {
                item_quantities.len() - 1
            } else {
                0
            };
            for (k, &v) in &item_quantities {
                output.push_str(&Green.paint(k.to_string()).to_string());
                if v > 1 {
                    output.push_str(&(" (".to_owned() + &v.to_string() + ")"));
                }
                if commas > 0 {
                    output.push_str(", ");
                    commas -= 1;
                }
            }

            println!("Items: {}", output);
        }
    }
}

pub fn ctake(command: &str, player: &mut Player, current_room: &mut Room) {
    if command.starts_with("ctake?") {
        println!("Take items from a given container. The container name is provided first, then a list of item names follows.");
        println!("Usage: ctake <container> <item> {{item}}");
        println!("Tip: in place of an item, you can say {}, which will assume every item in the container at once.", Green.paint("all"));
    } else {
        let words = split_whitespace_with_quotes(&filter_text(command));
        if words.len() < 3 {
            println!("{}", words.len());
            println!("Usage: `ctake <container> <items>` where items can be one or a list of item names to take from the given container.");
        } else {
            let mut words = words.iter();
            words.next().unwrap(); // consume "ctake"
            let container_name = words.next().unwrap();

            let mut inventory: Option<&mut Container> = None;
            let mut found_container = false;

            for container in &mut current_room.containers {
                if &filter_text(&container.name) == container_name {
                    inventory = Some(container);
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

            let cinventory = match inventory { Some(ref c) => c.items.as_slice(), None => &player.items() };

            let mut items = Vec::<&Item>::new();
            for &item in &item_names {
                if item == "all" {
                    items.extend(cinventory);
                    break;
                }

                let mut found_item = false;
                for &citem in cinventory {
                    if item == &filter_text(&citem.name()) {
                        items.push(citem);
                        found_item = true;
                    }
                }
                if !found_item {
                    println!("Could not find item {:?}", item);
                }
            }

            std::mem::drop(cinventory);

            match inventory {
                Some(c) => {
                    for item in &items {
                        c.items.remove_item(item);
                    }
                }
                None => {
                    for &item in &items {
                        player.remove_item(item);
                    }
                }
            }

            player.add_items(&items);
        }
    }
}

pub fn take(command: &str, player: &mut Player, current_room: &mut Room) {
    if command.starts_with("take?") {
        println!("Take items from the room.");
        println!("Usage: take <item> {{item}}");
        println!("Tip: in place of an item, you can say {}, which will assume every item in the room at once.", Green.paint("all"));
    } else {
        let item_names: Vec<String> = split_whitespace_with_quotes(&filter_text(command));
        if item_names.len() >= 2 {
            let mut items = Vec::<&Item>::new();
            for item in &item_names[1..] {
                if item == "all" {
                    items.extend(&current_room.items);
                    break;
                }

                let mut found_item = false;
                for &room_item in &current_room.items {
                    if item == &filter_text(room_item.name()) {
                        items.push(room_item);
                        found_item = true;
                    }
                }
                if !found_item {
                    println!("Could not find item {:?}", item);
                }
            }

            player.add_items(&items);

            // remove items from room
            for item in &items {
                current_room.items.remove_item(item);
            }

            println!("Took {} item{}", items.len(), if items.len() != 1 { 's' } else { ' ' });
        } else {
            println!("Usage: `take <items>` where `items` is a list of items in the room to pickup.");
        }
    }
}
