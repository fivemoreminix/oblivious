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

pub fn process_command(command: &str, player: &mut Player, current_room: &mut Room) {
    let cmd = command.to_lowercase();
    if cmd.starts_with("?") || cmd.contains("help") {
        println!("Commands: {}", list_options(&["look", "inventory [container]", "take <items>", "ctake <container> <items>"]));
        println!("Tip: For multi-word arguments, use quotation marks. E.x. take \"iron sword\"");
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
        let words: Vec<String> = split_whitespace_with_quotes(&cmd);
        let mut inventory: &Container = &player.inventory;
        
        if words.len() > 1 {
            let container_name = words[1].to_lowercase();
            let mut found_container = false;

            for container in &current_room.containers {
                if filter_text(&container.name) == container_name {
                    inventory = container;
                    found_container = true;
                }
            }

            if !found_container {
                println!("Could not find container {:?}", container_name);
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
            let mut item_quantities = HashMap::new();
            for item in inventory.items.iter().map(|item| item.name()) {
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
    } else if cmd.starts_with("ctake") {
        let words = split_whitespace_with_quotes(&cmd);
        if words.len() < 3 {
            println!("{}", words.len());
            println!("Usage: `ctake <container> <items>` where items can be one or a list of item names to take from the given container.");
        } else {
            let mut words = words.iter();
            words.next().unwrap(); // consume "ctake"
            let container_name = words.next().unwrap();

            let mut inventory: &mut Container = &mut player.inventory;
            let mut found_container = false;

            for container in &mut current_room.containers {
                if &filter_text(&container.name) == container_name {
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
            for &item in &item_names {
                let mut found_item = false;
                for &citem in &inventory.items {
                    if item == &filter_text(&citem.name()) {
                        items.push(citem);
                        found_item = true;
                    }
                }
                if !found_item {
                    println!("Could not find item {:?}", item);
                }
            }

            // remove items from container
            for item in &items {
                inventory.items.remove_item(item);
            }

            player.inventory.items.extend(&items);

            println!("Took {} item{}", items.len(), if items.len() > 1 { 's' } else { ' ' });
        }
    } else if cmd.starts_with("take") {
        let item_names: Vec<String> = split_whitespace_with_quotes(&cmd);
        if item_names.len() >= 2 {
            let mut items = Vec::<&Item>::new();
            for item in &item_names[1..] {
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

            player.inventory.items.extend(&items);

            // remove items from room
            for item in &items {
                current_room.items.remove_item(item);
            }

            println!("Took {} item{}", items.len(), if items.len() != 1 { 's' } else { ' ' });
        } else {
            println!("Usage: `take <items>` where `items` is a list of items in the room to pickup.");
        }
    } else {
        println!("Unrecognized command. Try 'help' or '?' for a list of commands.");
    }
}
