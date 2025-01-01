#include "user_input.h"


bool is_mouse_button_down(UserInput *input, MouseButton mouse_button) {
    if(input->mouse_button_down_handled) {
        return false;
    }
    else {
        input->mouse_button_down_handled = true;
        return IsMouseButtonDown(mouse_button);
    }
}

bool is_mouse_button_pressed(UserInput *input, MouseButton mouse_button) {
    if(input->mouse_button_pressed_handled) {
        return false;
    }
    else {
        input->mouse_button_pressed_handled = true;
        return IsMouseButtonPressed(mouse_button);
    }
}

bool is_mouse_button_released(UserInput *input, MouseButton mouse_button) {
    if(input->mouse_button_released_handled) {
        return false;
    }
    else {
        input->mouse_button_released_handled = true;
        return IsMouseButtonReleased(mouse_button);
    }
}

bool is_key_down(UserInput *input, KeyboardKey key) {
    if(input->key_down_handled) {
        return false;
    }
    else {
        input->key_down_handled = true;
        return IsKeyDown(key);
    }
}

bool is_key_pressed(UserInput *input, KeyboardKey key) {
    if(input->key_pressed_handled) {
        return false;
    }
    else {
        input->key_pressed_handled = true;
        return IsKeyPressed(key);
    }
}
