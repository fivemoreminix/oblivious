#![feature(duration_float)]

extern crate read_input;
extern crate ansi_term;

use read_input::*;
use ansi_term::Colour::*;
use ansi_term::Style;

use std::io::Write;
use std::time::Duration;

fn wait(time: Duration) {
    std::thread::sleep(time);
}

static WPM: f64 = 200.;
static WPS: f64 = WPM / 60.;
fn seconds_to_read(text: &str) -> f64 {
    text.split_whitespace().count() as f64 / WPS
}

fn narrate(text: &str) {
    println!("{}", Blue.paint(text));
    wait(Duration::from_float_secs(seconds_to_read(text)));
}

fn dialog(name: &str, text: &str) {
    println!("{}: {}", Red.paint(name), text);
    wait(Duration::from_float_secs(seconds_to_read(text)));
}

enum Race {
    HighElf, Argonian, WoodElf, Breton, DarkElf, Imperial, Khajit, Nord, Orc, Redguard,
}

fn main() {
    if cfg!(target = "windows") {
        match ansi_term::enable_ansi_support() {
            Err(_) => println!("NOTE: You will not have styled text. This is due to your terminal or operating system.\n"),
            _ => {}
        }
    }

    if false {
    narrate("An Imperial wagon is driving four prisoners down a snowy mountain pass. All are seated and bound; the one dressed in finery is gagged.");
    dialog("Ralof", "Hey, you. You're finally awake. You were trying to cross the border, right? Walked right into that Imperial ambush, same as us, and that thief over there.");
    dialog("Lokir", "Damn you Stormcloaks. Skyrim was fine until you came along. Empire was nice and lazy. If they hadn't been looking for you, I could've stolen that horse and been half way to Hammerfell. You there. You and me -- we should be here. It's these Stormcloaks the Empire wants.");
    dialog("Ralof", "We're all brothers and sisters in binds now, thief.");
    dialog("Imperial Soldier", "Shut up back there!");
    narrate("Lokir looks at the gagged man.");
    dialog("Lokir", "And what's wrong with him?");
    dialog("Ralof", "Watch your tongue! You're speaking to Ulfric Stormcloak, the true High King.");
    dialog("Lokir", "Ulfric? The Jarl of Windhelm? You're the leader of the rebellion. But if they captured you... Oh gods, where are they taking us?");
    dialog("Ralof", "I don't know where we're going, but Sovngarde awaits.");
    dialog("Lokir", "No, this can't be happening. This isn't happening.");
    dialog("Ralof", "Hey, what village are you from, horse thief?");
    dialog("Lokir", "Why do you care?");
    dialog("Ralof", "A Nord's last thoughts should be of home.");
    dialog("Lokir", "Rorikstead. I'm...I'm from Rorikstead.");
    narrate("They approach the village of Helgen. A soldier calls out to the lead wagon.");
    dialog("Imperial Soldier", "General Tullius, sir! The headsman is waiting!");
    dialog("General Tullius", "Good. Let's get this over with.");
    dialog("Lokir", "Shor, Mara, Dibella, Kynareth, Akatosh. Divines, please help me.");
    dialog("Ralof", "Look at him, General Tullius the Military Governor. And it looks like the Thalmor are with him. Damn elves. I bet they had something to do with this.\nThis is Helgen. I used to be sweet on a girl from here. Wonder if Vilod is still making that mead with juniper berries mixed in. Funny...when I was a boy, Imperial walls and towers used to make me feel so safe.");
    narrate("A man and son watch the prisoners pull into town.");
    dialog("Haming", "Who are they, daddy? Where are they going?");
    dialog("Torolf", "You need to go inside, little cub.");
    dialog("Haming", "Why? I want to watch the soldiers.");
    dialog("Torolf", "Inside the house. Now.");
    narrate("The wagon stops near the chopping block.");
    dialog("Imperial Soldier", "Whoa.");
    dialog("Lokir", "Why are they stopping?");
    dialog("Ralof", "Why do you think? End of the line. Let's go. Shouldn't keep the gods waiting for us.");
    dialog("Lokir", "No! Wait! We're not rebels!");
    dialog("Ralof", "Face your death with some courage, thief.");
    narrate("Under the Imperials' watchful eye, the prisoners start jumping out.");
    dialog("Lokir", "You've got to tell them! We weren't with you! This is a mistake!");
    dialog("Imperial Captain", "Step toward the block when we call your name. One at a time!");
    dialog("Ralof", "Empire loves their damn lists.");
    dialog("Hadvar", "Ulfric Stormcloak. Jarl of Windhelm.");
    dialog("Ralof", "It has been an honor, Jarl Ulfric!");
    dialog("Hadvar", "Ralof of Riverwood. Lokir of Rorikstead.");
    dialog("Lokir", "No, I'm not a rebel! You can't do this!");
    narrate("Lokir makes a break for it.");
    dialog("Imperial Captain", "Halt!");
    dialog("Lokir", "You're not going to kill me!");
    dialog("Imperial Captain", "Archers!");
    narrate("Lokir is downed with one bowshot.");
    dialog("Imperial Captain", "Anyone else feel like running?");
    dialog("Hadvar", "Wait, you there. Step forward. Who are you?");
    }

    // player customization //
    loop {
        let name: String = input_new().repeat_msg("Name: ").get();
        println!("{}", Green.paint(name));
        let yn: char = input_new().repeat_msg(&format!("Keep (y/n)? ")).add_test(|&c| c == 'Y' || c == 'y' || c == 'N' || c == 'n').get();
        match yn {
            'Y' | 'y' => break,
            'N' | 'n' => continue,
            _ => unreachable!(),
        }
    }

    let mut race = Race::HighElf;
    loop {
        let race_input: String = input_new().repeat_msg(&format!("Race ({}): ", Yellow.paint("?"))).get();
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
                for r in &["High Elf", "Argonian", "Wood Elf", "Breton", "Dark Elf", "Imperial", "Khajit", "Nord", "Orc", "Redguard"] {
                    print!("{}, ", Green.paint(*r));
                }
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
        println!("{}: {}", Style::new().bold().paint("Description"), description);

        let yn: char = input_new().repeat_msg(&format!("Keep (y/n)? ")).add_test(|&c| c == 'Y' || c == 'y' || c == 'N' || c == 'n').get();
        match yn {
            'Y' | 'y' => break,
            'N' | 'n' => continue,
            _ => unreachable!(),
        }
    }

    println!("\nGame script from http://www.gamershell.com/faqs/theelderscrollsvskyrimgamescript/");
}
