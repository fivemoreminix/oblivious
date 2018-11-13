#![feature(duration_float)]

extern crate read_input;

use read_input::*;

use std::time::Duration;

fn wait(time: Duration) {
    std::thread::sleep(time);
}

static WPM: f64 = 200.;
fn seconds_to_read(text: &str) -> f64 {
    text.split_whitespace().count() as f64 / (WPM / 60f64)
}

fn narrate(text: &str) {
    println!("{}", text);
    wait(Duration::from_float_secs(seconds_to_read(text)));
}

fn dialog(name: &str, text: &str) {
    println!("{}: {}", name, text);
    wait(Duration::from_float_secs(seconds_to_read(text)));
}

fn main() {
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

    // player customization

    println!("\nGame script from http://www.gamershell.com/faqs/theelderscrollsvskyrimgamescript/");
}
