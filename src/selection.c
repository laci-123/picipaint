#include "selection.h"
#include "assert.h"


void Selection_update(Camera2D camera, Object_array *objects) {
    assert(objects);

    if(IsKeyPressed(KEY_ESCAPE)) {
        for(size_t i = 0; i < objects->size; ++i) {
            Object_set_selection(&objects->items[i], false);
        }
        return;
    }

    if((IsKeyDown(KEY_LEFT_CONTROL) || IsKeyDown(KEY_RIGHT_CONTROL)) && IsKeyPressed(KEY_A)) {
        for(size_t i = 0; i < objects->size; ++i) {
            Object_set_selection(&objects->items[i], true);
        }
        return;
    }

    if(!IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
        return;
    }

    Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
    bool is_shift_down = IsKeyDown(KEY_LEFT_SHIFT) || IsKeyDown(KEY_RIGHT_SHIFT);
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
