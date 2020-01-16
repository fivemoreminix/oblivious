#include "game.h"

int main() {
    printf(
"\n         /\\       /\\\n"
"        / (       ) \\\n"
"       / /\\)     (/\\ \\\n"
"      / /   (\\      \\ \\\n"
"     / /    ; \\_/)   \\ \\\n"
"    / /    (,-. (     \\ \\\n"
"   / /         ) )     \\ \\\n"
"  / /       ,-','       \\ \\   Skyrim Text Adventure Game\n"
" / /     (\\(  (  /)      \\ \\                                  by Luke I. Wilson\n"
"/  '._____)\\)  \\/(______,'  \\__________________________________________________\n"
"\\                           /¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯¯\n"
" \\     .-.         ,-.     /  Version %s\n"
"  \\   /   \\/\\   (\\/   \\   /\n"
"   \\  \\      \\   )    /  /    Game script from\n"
"    \\  \\      ) /   ,'  /     http://www.gamershell.com/faqs/theelderscrollsvskyrimgamescript/\n"
"     \\  \\    / /   |   /\n"
"      \\  \\   \\ \\   |  /       All code is owned by me and licensed Creative-Commons Zero (CC0)\n"
"       \\  )   ) )  | /\n"
"        \\ |  / /   (/\n"
"         \\) / /\n"
"           / /  /|\n"
"           \\ \\_/ )\n"
"            \\   /\n"
"             \\_/\n",
    VERSION);

    printf("Press any key to start ...");
    getchar();

    level0_begin();

    return 0;
}
