#include "game.h"

unsigned words_in_str(char* const text) {
    unsigned words = 1;
    for (char* c = text; *c != '\0'; ++c)
        if (*c == ' ') ++words; // this function counts spaces in a string
    return words;              // therefore, dialog should avoid multiple spaces
}
