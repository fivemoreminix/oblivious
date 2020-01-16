#include "game.h"

int main() {
    printf(
    "         /\\       /\\\n"
    "        / (       ) \\\n"
    "       / /\\)     (/\\ \\\n"
    "      / /   (\\      \\ \\\n"
    "     / /    ; \\_/)   \\ \\\n"
    "    / /    (,-. (     \\ \\\n"
    "   / /         ) )     \\ \\\n"
    "  / /       ,-','       \\ \\   Skyrim Text Adventure Game \"Oblivious\"\n"
    " / /     (\\(  (  /)      \\ \\                                     by Luke I. Wilson\n"
    "/  '._____)\\)  \\/(______,'  \\__________________________________________________________________\n"
    "\\                           /¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯\n"
    " \\     .-.         ,-.     /  Version %s\n"
    "  \\   /   \\/\\   (\\/   \\   /\n"
    "   \\  \\      \\   )    /  /    Game script from\n"
    "    \\  \\      ) /   ,'  /     http://www.gamershell.com/faqs/theelderscrollsvskyrimgamescript/\n"
    "     \\  \\    / /   |   /\n"
    "      \\  \\   \\ \\   |  /       All code is owned by me and licensed Creative-Commons Zero (CC0)\n"
    "       \\  )   ) )  | /        The script and game is copyrighted content owned by Bethesda\n"
    "        \\ |  / /   (/\n"
    "         \\) / /\n"
    "           / /  /|\n"
    "           \\ \\_/ )\n"
    "            \\   /\n"
    "             \\_/\n", VERSION);

    printf("Press any key to start ...");
    getchar();

    NARRATE("An Imperial wagon is driving four prisoners down a snowy mountain pass. All are seated and bound; the one dressed in finery is gagged.");
    DIALOG("Ralof", "Hey, you. You're finally awake. You were trying to cross the border, right? Walked right into that Imperial ambush, same as us, and that thief over there.");
    DIALOG("Lokir", "Damn you Stormcloaks. Skyrim was fine until you came along. Empire was nice and lazy. If they hadn't been looking for you, I could've stolen that horse and been half way to Hammerfell. You there. You and me -- we should be here. It's these Stormcloaks the Empire wants.");
    DIALOG("Ralof", "We're all brothers and sisters in binds now, thief.");
    DIALOG("Imperial Soldier", "Shut up back there!");
    NARRATE("Lokir looks at the gagged man.");
    DIALOG("Lokir", "And what's wrong with him?");
    DIALOG("Ralof", "Watch your tongue! You're speaking to Ulfric Stormcloak, the true High King.");
    DIALOG("Lokir", "Ulfric? The Jarl of Windhelm? You're the leader of the rebellion. But if they captured you... Oh gods, where are they taking us?");
    DIALOG("Ralof", "I don't know where we're going, but Sovngarde awaits.");
    DIALOG("Lokir", "No, this can't be happening. This isn't happening.");
    DIALOG("Ralof", "Hey, what village are you from, horse thief?");
    DIALOG("Lokir", "Why do you care?");
    DIALOG("Ralof", "A Nord's last thoughts should be of home.");
    DIALOG("Lokir", "Rorikstead. I'm... I'm from Rorikstead.");
    NARRATE("They approach the village of Helgen. A soldier calls out to the lead wagon.");
    DIALOG("Imperial Soldier", "General Tullius, sir! The headsman is waiting!");
    DIALOG("General Tullius", "Good. Let's get this over with.");
    DIALOG("Lokir", "Shor, Mara, Dibella, Kynareth, Akatosh. Divines, please help me.");
    DIALOG("Ralof", "Look at him, General Tullius the Military Governor. And it looks like the Thalmor are with him. Damn elves. I bet they had something to do with this.\nThis is Helgen. I used to be sweet on a girl from here. Wonder if Vilod is still making that mead with juniper berries mixed in. Funny...when I was a boy, Imperial walls and towers used to make me feel so safe.");
    NARRATE("A man and son watch the prisoners pull into town.");
    DIALOG("Haming", "Who are they, daddy? Where are they going?");
    DIALOG("Torolf", "You need to go inside, little cub.");
    DIALOG("Haming", "Why? I want to watch the soldiers.");
    DIALOG("Torolf", "Inside the house. Now.");
    NARRATE("The wagon stops near the chopping block.");
    DIALOG("Imperial Soldier", "Whoa.");
    WAIT(2);
    DIALOG("Lokir", "Why are they stopping?");
    DIALOG("Ralof", "Why do you think? End of the line. Let's go. Shouldn't keep the gods waiting for us.");
    DIALOG("Lokir", "No! Wait! We're not rebels!");
    DIALOG("Ralof", "Face your death with some courage, thief.");
    NARRATE("Under the Imperials' watchful eye, the prisoners start jumping out.");
    DIALOG("Lokir", "You've got to tell them! We weren't with you! This is a mistake!");
    DIALOG("Imperial Captain", "Step toward the block when we call your name. One at a time!");
    DIALOG("Ralof", "Empire loves their damn lists.");
    DIALOG("Hadvar", "Ulfric Stormcloak. Jarl of Windhelm.");
    DIALOG("Ralof", "It has been an honor, Jarl Ulfric!");
    DIALOG("Hadvar", "Ralof of Riverwood. Lokir of Rorikstead.");
    DIALOG("Lokir", "No, I'm not a rebel! You can't do this!");
    NARRATE("Lokir makes a break for it.");
    DIALOG("Imperial Captain", "Halt!");
    DIALOG("Lokir", "You're not going to kill me!");
    DIALOG("Imperial Captain", "Archers!");
    WAIT(2);
    NARRATE("Lokir is downed with one bowshot."); DIALOG("Imperial Captain", "Anyone else feel like running?");
    DIALOG("Hadvar", "Wait, you there. Step forward. Who are you?");

    return 0;
}
