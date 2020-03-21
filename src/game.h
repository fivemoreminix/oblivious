#ifndef GAME_H
#define GAME_H

#include <stdio.h>
#include <string.h>

#ifdef _WIN32
#include <Windows.h>
#define WAIT(sec) Sleep(sec*1000)
#else
#include <unistd.h>
#define WAIT(sec) sleep(sec)
#endif

#define VERSION "0.1"
#define WPM (300)

unsigned words_in_str(char* text);

#define WPS (WPM / 60)
#define SEC_TO_READ(text) (words_in_str(text) / WPS)
void narrate(char* const text);
void dialog(char* const name, char* const text);

typedef struct game_settings game_settings;
struct game_settings {
    int wpm;
};
int string_to_game_settings(game_settings* gs, char* const input); /* errors: 0 = no error, 1 = an error found while parsing */
int game_settings_to_string(char* str, game_settings* const gs);

void level0_begin();

#endif /* GAME_H */

