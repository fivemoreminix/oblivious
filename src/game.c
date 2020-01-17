#include "game.h"

unsigned words_in_str(char* const text) {
    unsigned words = 1;
    for (char* c = text; *c != '\0'; ++c)
        if (*c == ' ') ++words; // this function counts spaces in a string
    return words;              // therefore, dialog should avoid multiple spaces
}

/* Pass an allocated game_settings */
int string_to_game_settings(char* const input, game_settings* gs) {
    gs->wpm = 300;
    return 0;
}

char* game_settings_to_string(game_settings* const gs) {
    char* outp = "wpm: ";
    return strcat(outp, itoa(gs->wpm));
}
