#include "game.h"

void narrate(char* const text) {
    printf("*%s*\n", text);
    WAIT(SEC_TO_READ(text) + 1.0);
}
void dialog(char* const name, char* const text) {
    printf("%s: %s\n", name, text);
    WAIT(SEC_TO_READ(text) + 1.0);
}

unsigned words_in_str(char* const text) {
    unsigned words = 1;
    for (char* c = text; *c != '\0'; ++c)
        if (*c == ' ') ++words; // this function counts spaces in a string
    return words;              // therefore, dialog should avoid multiple spaces
}

/* Pass an allocated game_settings */
int string_to_game_settings(game_settings* gs, char* const input) {
    gs->wpm = 300;
    return 0;
}

int game_settings_to_string(char* str, game_settings* const gs) {
    return sprintf(str, "wpm: %d\n", gs->wpm);
}
