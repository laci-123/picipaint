#include "selection.h"
#include "assert.h"


static void update_movement(Camera2D camera, Object_array *objects, UserInput *input) {
    if(is_mouse_button_down(input, MOUSE_BUTTON_LEFT)) {
        Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
        bool mouse_is_over_a_selected = false;
        for(size_t i = 0; i < objects->size; ++i) {
            if(Object_is_selected(&objects->items[i]) && Object_is_under_mouse(mouse_pos, &objects->items[i])) {
                mouse_is_over_a_selected = true;
                break;
            }
        }
        Vector2 mouse_delta = Vector2Scale(GetMouseDelta(), 1.0f / camera.zoom);
        for(size_t i = 0; i < objects->size; ++i) {
            if(Object_is_selected(&objects->items[i])) {
                if(Object_is_moved(&objects->items[i])) {
                    Object_move(mouse_delta, &objects->items[i]);
                }
                else if(mouse_is_over_a_selected) {
                    Object_set_moving(&objects->items[i], true);
                }
            }
        }
    }
    else {
        for(size_t i = 0; i < objects->size; ++i) {
            Object_set_moving(&objects->items[i], false);
        }
    }
}


static void update_deletion(Object_array *objects, UserInput *input) {
    if(is_key_pressed(input, KEY_DELETE)) {
        for(size_t i = 0; i < objects->size; ++i) {
            if(Object_is_selected(&objects->items[i])) {
                Object deleted = Object_array_delete(objects, i--); // array_delete messes with the indexes, be careful!
                Object_free(&deleted);
            }
        }
        Object_array_shrink_to_fit(objects);
    }
}


void Selection_update(Camera2D camera, Object_array *objects, UserInput *input) {
    assert(objects);

    update_movement(camera, objects, input);
    update_deletion(objects, input);

    if(is_key_pressed(input, KEY_ESCAPE)) {
        for(size_t i = 0; i < objects->size; ++i) {
            Object_set_selection(&objects->items[i], false);
        }
        return;
    }

    if((is_key_down(input, KEY_LEFT_CONTROL) || is_key_down(input, KEY_RIGHT_CONTROL)) && is_key_pressed(input, KEY_A)) {
        for(size_t i = 0; i < objects->size; ++i) {
            Object_set_selection(&objects->items[i], true);
        }
        return;
    }

    if(!is_mouse_button_released(input, MOUSE_BUTTON_LEFT)) {
        return;
    }

    Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
    bool is_shift_down = is_key_down(input, KEY_LEFT_SHIFT) || is_key_down(input, KEY_RIGHT_SHIFT);
    for(size_t i = 0; i < objects->size; ++i) {
        if(Object_is_under_mouse(mouse_pos, &objects->items[i])) {
            if(is_shift_down) {
                Object_set_selection(&objects->items[i], !Object_is_selected(&objects->items[i]));
            }
            else {
                Object_set_selection(&objects->items[i], true);
            }
        }
        else if(!is_shift_down) {
            Object_set_selection(&objects->items[i], false);
        }
    }
}
