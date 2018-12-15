use crate::definitions::*;

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
