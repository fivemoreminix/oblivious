#ifndef GAME_H
#define GAME_H

#include <stdio.h>

#ifdef _WIN32
#include <Windows.h>
#define WAIT(sec) Sleep(sec*1000)
#else
#include <unistd.h>
#define WAIT(sec) sleep(sec)
#endif

#define VERSION "0.1"
#define WPM (300)

unsigned words_in_str(char* text) {
    unsigned words = 1;
    for (char* c = text; *c != '\0'; ++c)
        if (*c == ' ') ++words; // this function counts spaces in a string
    return words;              // therefore, dialog should avoid multiple spaces
}

#define WPS (WPM / 60)
#define SEC_TO_READ(text) (words_in_str(text) / WPS)
#define NARRATE(text) printf("*%s*\n", text); \
    WAIT(SEC_TO_READ(text) + 1.0)
#define DIALOG(name, text) printf("%s: %s\n", name, text); \
    WAIT(SEC_TO_READ(text) + 1.0)

void level0_begin();

#endif /* GAME_H */

