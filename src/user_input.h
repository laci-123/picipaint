#ifndef USER_INPUT_INCLUDED_
#define USER_INPUT_INCLUDED_

#include "raypack.h"

typedef struct {
    bool mouse_button_down_handled;
    bool mouse_button_pressed_handled;
    bool mouse_button_released_handled;
    bool key_down_handled;
    bool key_pressed_handled;
} UserInput;

bool is_mouse_button_down(UserInput *input, MouseButton mouse_button);
bool is_mouse_button_pressed(UserInput *input, MouseButton mouse_button);
bool is_mouse_button_released(UserInput *input, MouseButton mouse_button);
bool is_key_down(UserInput *input, KeyboardKey key);
bool is_key_pressed(UserInput *input, KeyboardKey key);

#endif //USER_INPUT_INCLUDED_
