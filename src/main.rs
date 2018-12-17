#![feature(duration_float)]
#![feature(vec_remove_item)]
#![allow(dead_code)]
#![allow(unused_assignments)]

#[macro_use]
extern crate version;
extern crate ansi_term;
extern crate read_input;

mod items;
mod definitions;
mod game;
mod player;

use self::items::*;
use self::items::apparel::*;
use self::definitions::*;
use self::game::*;
use self::player::*;

use ansi_term::Colour::*;
use ansi_term::Style;
use read_input::*;

use std::time::Duration;

fn main() {
    #[cfg(target_os = "windows")]
    match ansi_term::enable_ansi_support() {
        Err(_) => println!("NOTE: You will not have styled text. This is due to your terminal or operating system.\n"),
        _ => {}
    }

    // print skyrim logo
    println!(
r#"
         /\       /\
        / (       ) \
       / /\)     (/\ \
      / /   (\      \ \
     / /    ; \_/)   \ \
    / /    (,-. (     \ \
   / /         ) )     \ \
  / /       ,-','       \ \   Skyrim Text Adventure Game "{}"
 / /     (\(  (  /)      \ \                                     by {}
/  '._____)\)  \/(______,'  \__________________________________________________
\                           /¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯
 \     .-.         ,-.     /  Version {}
  \   /   \/\   (\/   \   /
   \  \      \   )    /  /    Game script from
    \  \      ) /   ,'  /     {}
     \  \    / /   |   /
      \  \   \ \   |  /       All code is owned by me and licensed Creative-Commons Zero (CC0)
       \  )   ) )  | /        The script and game is copyrighted content owned by Bethesda
        \ |  / /   (/
         \) / /
           / /  /|
           \ \_/ )
            \   /
             \_/
"#, Green.paint("Oblivious"), Cyan.paint("Luke Wilson"), Green.paint(version!()), Style::new().underline().paint("http://www.gamershell.com/faqs/theelderscrollsvskyrimgamescript/"));

    input_new::<String>().msg("Press enter to start... ").get();

    prologue();
}

fn prologue() {
    if true {
        narrate("An Imperial wagon is driving four prisoners down a snowy mountain pass. All are seated and bound; the one dressed in finery is gagged.");
        dialog("Ralof", "Hey, you. You're finally awake. You were trying to cross the border, right? Walked right into that Imperial ambush, same as us, and that thief over there.");
        dialog("Lokir", "Damn you Stormcloaks. Skyrim was fine until you came along. Empire was nice and lazy. If they hadn't been looking for you, I could've stolen that horse and been half way to Hammerfell. You there. You and me -- we should be here. It's these Stormcloaks the Empire wants.");
        dialog(
            "Ralof",
            "We're all brothers and sisters in binds now, thief.",
        );
        dialog("Imperial Soldier", "Shut up back there!");
        narrate("Lokir looks at the gagged man.");
        dialog("Lokir", "And what's wrong with him?");
        dialog(
            "Ralof",
            "Watch your tongue! You're speaking to Ulfric Stormcloak, the true High King.",
        );
        dialog("Lokir", "Ulfric? The Jarl of Windhelm? You're the leader of the rebellion. But if they captured you... Oh gods, where are they taking us?");
        dialog(
            "Ralof",
            "I don't know where we're going, but Sovngarde awaits.",
        );
        dialog(
            "Lokir",
            "No, this can't be happening. This isn't happening.",
        );
        dialog("Ralof", "Hey, what village are you from, horse thief?");
        dialog("Lokir", "Why do you care?");
        dialog("Ralof", "A Nord's last thoughts should be of home.");
        dialog("Lokir", "Rorikstead. I'm... I'm from Rorikstead.");
        narrate("They approach the village of Helgen. A soldier calls out to the lead wagon.");
        dialog(
            "Imperial Soldier",
            "General Tullius, sir! The headsman is waiting!",
        );
        dialog("General Tullius", "Good. Let's get this over with.");
        dialog(
            "Lokir",
            "Shor, Mara, Dibella, Kynareth, Akatosh. Divines, please help me.",
        );
        dialog("Ralof", "Look at him, General Tullius the Military Governor. And it looks like the Thalmor are with him. Damn elves. I bet they had something to do with this.\nThis is Helgen. I used to be sweet on a girl from here. Wonder if Vilod is still making that mead with juniper berries mixed in. Funny...when I was a boy, Imperial walls and towers used to make me feel so safe.");
        narrate("A man and son watch the prisoners pull into town.");
        dialog("Haming", "Who are they, daddy? Where are they going?");
        dialog("Torolf", "You need to go inside, little cub.");
        dialog("Haming", "Why? I want to watch the soldiers.");
        dialog("Torolf", "Inside the house. Now.");
        narrate("The wagon stops near the chopping block.");
        dialog("Imperial Soldier", "Whoa.");
        wait(Duration::from_secs(2));
        dialog("Lokir", "Why are they stopping?");
        dialog(
            "Ralof",
            "Why do you think? End of the line. Let's go. Shouldn't keep the gods waiting for us.",
        );
        dialog("Lokir", "No! Wait! We're not rebels!");
        dialog("Ralof", "Face your death with some courage, thief.");
        narrate("Under the Imperials' watchful eye, the prisoners start jumping out.");
        dialog(
            "Lokir",
            "You've got to tell them! We weren't with you! This is a mistake!",
        );
        dialog(
            "Imperial Captain",
            "Step toward the block when we call your name. One at a time!",
        );
        dialog("Ralof", "Empire loves their damn lists.");
        dialog("Hadvar", "Ulfric Stormcloak. Jarl of Windhelm.");
        dialog("Ralof", "It has been an honor, Jarl Ulfric!");
        dialog("Hadvar", "Ralof of Riverwood. Lokir of Rorikstead.");
        dialog("Lokir", "No, I'm not a rebel! You can't do this!");
        narrate("Lokir makes a break for it.");
        dialog("Imperial Captain", "Halt!");
        dialog("Lokir", "You're not going to kill me!");
        dialog("Imperial Captain", "Archers!");
        wait(Duration::from_secs(2));
        narrate("Lokir is downed with one bowshot.");
        dialog("Imperial Captain", "Anyone else feel like running?");
        dialog("Hadvar", "Wait, you there. Step forward. Who are you?");
    }

    // player customization //
    println!("");
    let mut gender = Gender::Male;
    loop {
        match &input_new::<String>()
            .repeat_msg(&format!(
                "Gender ({}): ",
                list_options(&["(m)ale", "(f)emale"])
            ))
            .get()
            .trim()
            .to_lowercase()[..]
        {
            "m" | "male" => {
                gender = Gender::Male;
                break;
            }
            "f" | "female" => {
                gender = Gender::Female;
                break;
            }
            _ => {
                println!(
                    "Not a valid gender. Choose male or female. Note: there are only two genders."
                );
            }
        }
    }

    let mut name: String;
    loop {
        name = input_new().repeat_msg("Name (first): ").get();
        println!("{}", Green.paint(&name));
        let yn: char = input_new()
            .repeat_msg(&format!("Keep (y/n)? "))
            .add_test(|&c| c == 'Y' || c == 'y' || c == 'N' || c == 'n')
            .get();
        match yn {
            'Y' | 'y' => break,
            'N' | 'n' => continue,
            _ => unreachable!(),
        }
    }

    let mut race = Race::HighElf;
    loop {
        let race_input: String = input_new()
            .repeat_msg(&format!("Race ({}): ", Yellow.paint("?")))
            .get();
        match &race_input.trim().to_lowercase()[..] {
            "high elf" => race = Race::HighElf,
            "argonian" => race = Race::Argonian,
            "wood elf" => race = Race::WoodElf,
            "breton" => race = Race::Breton,
            "dark elf" => race = Race::DarkElf,
            "imperial" => race = Race::Imperial,
            "khajit" => race = Race::Khajit,
            "nord" => race = Race::Nord,
            "orc" => race = Race::Orc,
            "redguard" => race = Race::Redguard,
            "?" | "" => {
                println!(
                    "{}",
                    list_options(&[
                        "High Elf", "Argonian", "Wood Elf", "Breton", "Dark Elf", "Imperial",
                        "Khajit", "Nord", "Orc", "Redguard"
                    ])
                );
                println!("");
                continue;
            }
            _ => {
                println!("Invalid race. Try '?' for a list of races.");
                continue;
            }
        }

        let description = match race {
            Race::HighElf => "Also known as \"Altmer\" in their homeland of Summerset Isle, the high elves are the most strongly gifted in the arcane arts of all the races. They can call upon their Highborn power to regenerate Magicka quickly.",
            Race::Argonian => "This reptilian race, well-suited for the treacherous swamps of their Black Marsh homeland, has developed a natural resistance to diseases and the ability to breathe underwater. They can call upon the Histskin to regenerate health very quickly.",
            Race::WoodElf => "The clanfolk of the Western Valenwood forests, also known as \"Bosmer.\" Wood elves make good scouts and thieves, and there are no finer archers in all of Tamriel. They have natural resistances to both poisons and diseases. They can Command Animals to fight for them.",
            Race::Breton => "In addition to their quick and perceptive grasp of spellcraft, even the humblest of High Rock's Bretons can boast a resistance to magic. Bretons can call upon the Dragonskin power to absorb spells.",
            Race::DarkElf => "Also known as \"Dunmer\" in their homeland of Morrowind, dark elves are noted for their stealth and magic skills. They are naturally resistant to fire and can call upon their Ancestor's Wrath to surround themselves in fire.",
            Race::Imperial => "Natives of Cyrodiil, they have proved to be shrewd diplomats and traders. They are skilled with combat and magic. Anywhere gold coins might be found, Imperials always seem to find a few more. They can call upon the Voice of the Emperor to calm an enemy.",
            Race::Khajit => "Hailing from the province of Elsweyr, they are intelligent, quick, and agile. They make excellent thieves due to their natural stealthiness. All Khajiit can see in the dark at will and have unarmed claw attacks.",
            Race::Nord => "Citizens of Skyrim, they are a tall and fair-haired people. Strong and hardy, Nords are famous for their resistance to cold and their talent as warriors. They can use a Battlecry to make opponents flee.",
            Race::Orc => "The people of the Wrothgarian and Dragontail Mountains, Orcish smiths are prized for their craftsmanship. Orc troops in Heavy Armor are among the finest in the Empire, and are fearsome when using their Berserker Rage.",
            Race::Redguard => "The most naturally talented warriors in Tamriel, the Redguards of Hammerfell have a hardy constitution and a natural resistance to poison. They can call upon an Adrenaline Rush in combat.",
        };
        println!(
            "{}: {}",
            Style::new().bold().paint("Description"),
            description
        );

        let yn: char = input_new()
            .repeat_msg(&format!("Keep (y/n)? "))
            .add_test(|&c| c == 'Y' || c == 'y' || c == 'N' || c == 'n')
            .get();
        match yn {
            'Y' | 'y' => break,
            'N' | 'n' => continue,
            _ => unreachable!(),
        }
    }
    println!(""); // end character customization with blank line

    let mut player = Player::new(&name, race, gender);
    player.inventory.items.push(&ROUGHSPUN_TUNIC);
    player.inventory.items.push(&FOOTWRAPS);

    if true {
        dialog("Hadvar", match race {
        Race::HighElf => "You're not with the Thalmor Embassy, are you, high elf? No, that can't be right...",
        Race::Argonian => "Are you a relative of one of the Riften dock workers, Argonian?",
        Race::WoodElf => "Not many wood elves would choose to come alone to Skyrim.",
        Race::Breton => "You from Daggerfall, Breton? Fleeing from some court intrigue?",
        Race::DarkElf => "Another refugee? Gods really have abandoned your people, dark elf.",
        Race::Imperial => "You're a long way from the Imperial City. What're you doing in Skyrim?",
        Race::Khajit => "You with one of the trade caravans, Khajiit? Your kind always seems to find trouble.",
        Race::Nord => "You picked a bad time to come home to Skyrim, kinsman.",
        Race::Orc => "You from one of the strongholds, Orc? How did you end up here?",
        Race::Redguard => "What are you doing here, Redguard? You a sellsword? A sailor from Stros M'kai?",
    });
        dialog(
            "Hadvar",
            &format!(
                "Captain, what should we do? {}'s not on the list.",
                gender.he_she(true)
            ),
        );
        dialog(
            "Imperial Captain",
            &format!(
                "Forget the list. {} goes to the block.",
                gender.he_she(true)
            ),
        );
        dialog("Hadvar", "By your orders, captain.");
        dialog(
            "Hadvar",
            match race {
                Race::HighElf => {
                    "I'm sorry. We'll make sure your remains are returned to the Summerset Isle."
                }
                Race::Argonian => {
                    "I'm sorry. We'll make sure your remains are returned to Black Marsh."
                }
                Race::WoodElf => {
                    "I'm sorry. We'll make sure your remains are returned to Valenwood."
                }
                Race::Breton => {
                    "I'm sorry. We'll make sure your remains are returned to High Rock."
                }
                Race::DarkElf => {
                    "I'm sorry. We'll make sure your remains are returned to Morrowind."
                }
                Race::Imperial => {
                    "I'm sorry. We'll make sure your remains are returned to Cyrodiil."
                }
                Race::Khajit => "I'm sorry. We'll make sure your remains are returned to Elsweyr.",
                Race::Nord => "I'm sorry. At least you'll die here, in your homeland.",
                Race::Orc => "I'm sorry. We'll make sure your remains are returned to Orsinium.",
                Race::Redguard => {
                    "I'm sorry. We'll make sure your remains are returned to Hammerfell."
                }
            },
        );
        dialog("Hadvar", "Follow the Captain, prisoner.");
        narrate("You go to stand with the other waiting prisoners by the block.");
        dialog("Tullius", "Ulfric Stormcloak. Some here in Helgen call you a hero, but a hero doesn't use a power like the Voice to murder his king and usurp his throne.");
        dialog("Jarl Ulfric Stormcloak", "(grunting protest)");
        dialog("Tullius", "You started this war, plunged Skyrim into chaos and now the Empire is going to put you down, and restore the peace.");
        narrate("A distant noise booms down the mountainside.");
        dialog("Hadvar", "What was that?");
        dialog("Tullius", "It's nothing. Carry on.");
        dialog(
            "Imperial Captain",
            "Yes, General Tullius. Give them their last rites.",
        );
        dialog("Priestess of Arkay", "As we commend your souls to Aetherius, blessings of the Eight Divines upon you, for you are the salt and earth of Nirn, our beloved--");
        narrate("One of the prisoners from the lead wagon walks forward.");
        dialog(
            "Stormcloak Soldier",
            "For the love of Talos, shut up and lets get this over with.",
        );
        dialog("Priestess of Arkay", "As you wish...");
        wait(Duration::from_secs(4));
        dialog("Stormcloak Soldier", "Come on, I haven't got all morning. My ancestors are smiling at me, Imperials. Can you say the same?");
        narrate("They behead the Stormcloak, eliciting responses from the onlookers.");
        dialog("Stormcloak Solider 2", "You Imperial bastards!");
        dialog("Vilod", "Justice!");
        dialog("Ingrid", "Death to the Stormcloaks!");
        dialog("Ralof", "As fearless in death as he was in life.");
        dialog(
            "Imperial Captain",
            match race {
                Race::HighElf => "Next, the high elf!",
                Race::Argonian => "Next, the lizard!",
                Race::WoodElf => "Next, the wood elf!",
                Race::Breton => "Next, the Breton!",
                Race::DarkElf => "Next, the dark elf!",
                Race::Imperial => "Next, the renegade from Cyrodiil!",
                Race::Khajit => "Next, the cat!",
                Race::Nord => "Next, the Nord in the rags!",
                Race::Orc => "Next, the Orc!",
                Race::Redguard => "Next, the Redguard!",
            },
        );
        narrate("Another cry rings out on the mountainside, this time much closer.");
        dialog("Hadvar", "There it is again. Did you hear that?");
        dialog("Imperial Captain", "I said, next prisoner!");
        dialog("Hadvar", "To the block, prisoner. Nice and easy.");
        narrate("The player is brought to the chopping block. A large creature swoops over the southern peaks, barreling toward Helgen.");
        dialog("Tullius", "What in Oblivion is that?");
        dialog("Imperial Captain", "Sentries! What do you see?");
        dialog("Imperial Soldier", "It's in the clouds!");
        narrate("The creature, now identified as a dragon, lands on the watchtower next to the block, rumbling the ground and surprising the onlookers.");
        dialog("Stormcloak Solider", "Dragon!");
        narrate("The dragon uses its voice on the crowd, killing the headsman.");
        dialog("Headsman", "Nngh!");
        dialog(
            "Tullius",
            "Don't just stand there, kill that thing! Guards, get the townspeople to safety!",
        );
        dialog(
            "Ralof",
            "Get up! Come on, the gods won't give us another chance!",
        );
        dialog("Ralof", "This way!");
        narrate("Ralof leads you to a tower where the other prisoners are hiding.");
        dialog(
            "Ralof",
            "Jarl Ulfric! What is that thing? Could the legends be true?",
        );
        dialog(
            "Ulfric",
            "Legends don't burn down villages. We need to move, now!",
        );
        dialog(
            "Ralof",
            "Up through the tower. Let's go! This way, friend! Move!",
        );
        dialog(
            "Stormcloak Soldier",
            "We just need to move some of these rocks to clear the way!",
        );
        narrate("The dragon, Alduin, breaks in through the wall, blasting fire everywhere.");
        dialog("Ralof", "Get back!");
        dialog("Alduin", "Toor shul!");
        wait(Duration::from_secs(3));
        narrate("They survey the damage when the dragon leaves.");
        dialog("Ralof", "See the inn on the other side? Jump through the roof and keep going! Go! We'll follow you when we can!");
    }

    loop {
        match &input_new::<String>().repeat_msg(">").get().trim().to_lowercase()[..] {
            "look" => println!("{}", Blue.paint("Standing forty feet above the ground, in front of a simmering hole in the tower's stone wall; you overlook a pillaged town, with burning buildings, people running frantically, fire, arrows, and the dragon's booming voice.")),
            "jump" => {
                narrate("You take a step back, size up your target, and jump from the side of the tower into the open roof of the inn below.");
                break;
            }
            "?" => println!("{}", list_options(&["look", "jump (continue story)"])),
            _ => println!("Command unknown. '?' for a list of commands."),
        }
    }

    narrate("You hit the ground running on the upstairs floor, and you continue running to where the floor has already fallen in and transitions into the bottom level floor.");
    narrate("You escape through the inn's door, encountering the list-giver with some refugees.");
    dialog("Hadvar", "Haming, you need to get over here now! Thataboy. You're doing great! Torolf! Gods...everyone get back!");
    narrate("The dragon breathes an immense blaze in your direction, but nobody is engulfed.");
    dialog("Hadvar", &format!("Still alive, prisoner? Keep close to me if you want to stay that way. Gunnar, take care of the {}. I have to find General Tullius and join the defense.", gender.boy_girl()));
    dialog("Gunnar", "Gods guide you, Hadvar.");
    dialog("Hadvar", "Stay close to the wall!");
    narrate("They take cover as a dragon perches right above.");
    dialog("Alduin", "Vol toor shul!");
    wait(Duration::from_secs(3));
    dialog("Hadvar", "Quickly, follow me!");
    narrate("They reach the main gate's carnage. Soldiers are firing arrows in vain.");
    dialog("Vilod", "Tell my family I fought bravely!");
    dialog("Tullius", "Hadvar! Into the keep, solider, we're leaving!");
    dialog("Hadvar", "It's you and me, prisoner, stay close!");
    dialog(
        "Imperial Soldier",
        "How in Oblivion do we kill this thing? Just...die!",
    );
    narrate("Near the keep, Ralof is seen.");
    dialog("Hadvar", "Ralof! You damned traitor, out of my way!");
    dialog(
        "Ralof",
        "We're escaping, Hadvar! You're not stopping us this time.",
    );
    dialog(
        "Hadvar",
        "Fine. I hope that dragon takes you all to Sovngarde.",
    );
    dialog("Ralof", "You, come on! Into the keep!");
    dialog(
        "Hadvar",
        "With me, prisoner! Let's go! Come on! We need to get inside!",
    );

    loop {
        match &input_new::<String>()
            .repeat_msg(&format!(
                "Choose ({}): ",
                list_options(&["Hadvar", "Ralof"])
            ))
            .get()
            .trim()
            .to_lowercase()[..]
        {
            "hadvar" => {
                narrate("Coming soon :)");
                break;
            }
            "ralof" => {
                branch_ralof(&mut player);
                break;
            }
            _ => println!("{}", Red.paint("Choose either Hadvar or Ralof.")),
        }
    }
}

fn branch_hadvar(player: &mut Player) {
    dialog("Hadvar", "Looks like we're the only ones who made it. Was that really a dragon? The bringers of the End Times? We should keep moving. Come here. Let me see if I can get those bindings off. There you go. Take a look around, there should be plenty of gear to choose from. I'm going to see if I can find something for these burns. You better get that armor on. Give sword a few swings, too. Let's keep moving. That thing is still out there. Come on, this way.")
    
}

fn branch_ralof(player: &mut Player) {
    narrate("Upon entering the keep, Ralof goes to check on a fallen comrade.");
    dialog("Ralof", "We'll meet again in Sovngarde, brother. Looks like we're the only ones who made it. That thing was a dragon. No doubt. Just like the children's stories and the legends. The harbingers of the End Times. We better get moving. Come here, let me see if I can get those bindings off. There you go. May as well take Gunjar's gear...he won't be needing it anymore. Alright, get that armor on and give that axe a few swings. I'm going to see if I can find some way out of here. This one's locked. Let's see about that gate. Damn. No way to open this from our side.");

    let chest = Container::new("Warden's Chest", vec![&IMPERIAL_LIGHT_ARMOR, &IRON_SWORD, &IMPERIAL_LIGHT_BOOTS, &HELGEN_KEEP_KEY]);
    let mut room = Room::new("Helgen's Keep", "Cold, rumbling stone walls lit by several torch flames. The dragon's rustling outside the keep causes particles to be shaken from the walls.", Some(vec!(&IRON_SWORD)), Some(vec!(chest)));

    loop {
        process_command(
            &input_new::<String>().repeat_msg(">").get().trim(),
            player,
            &mut room,
        );

        if player.inventory_weapons().contains(&&IMPERIAL_SWORD) {
            break;
        }
    }
}
