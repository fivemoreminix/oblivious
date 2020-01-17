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

    printf("\n1: Continue Game\n2: New Game\n3: Load Game\n\n0: Quit\n");
    int opt;
    if (scanf("%d", &opt) != 1) {
        // A number isn't entered
    } else {
        printf("Succeeded: %d\n", opt);
    }

    return 0;
}
