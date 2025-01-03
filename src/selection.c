#include "selection.h"
#include "assert.h"


static void update_movement(Camera2D camera, Object_array *objects) {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
        Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
        bool mouse_is_over_a_selected = false;
        for(size_t i = 0; i < objects->size; ++i) {
            if(objects->items[i].as.selectable.is_selected && Object_is_under_mouse(mouse_pos, &objects->items[i])) {
                mouse_is_over_a_selected = true;
                break;
            }
        }
        Vector2 mouse_delta = Vector2Scale(GetMouseDelta(), 1.0f / camera.zoom);
        for(size_t i = 0; i < objects->size; ++i) {
            if(objects->items[i].as.selectable.is_selected) {
                if(objects->items[i].as.selectable.is_moved) {
                    Object_move(mouse_delta, &objects->items[i]);
                }
                else if(mouse_is_over_a_selected) {
                    objects->items[i].as.selectable.is_moved = true;
                }
            }
        }
    }
    else {
        for(size_t i = 0; i < objects->size; ++i) {
            objects->items[i].as.selectable.is_moved = false;
        }
    }
}


static void update_deletion(Object_array *objects) {
    if(IsKeyPressed(KEY_DELETE)) {
        for(size_t i = 0; i < objects->size; ++i) {
            if(objects->items[i].as.selectable.is_selected) {
                Object deleted = Object_array_delete(objects, i--); // array_delete messes with the indexes, be careful!
                Object_free(&deleted);
            }
        }
        Object_array_shrink_to_fit(objects);
    }
}


void Selection_update(Camera2D camera, Object_array *objects) {
    assert(objects);

    update_movement(camera, objects);
    update_deletion(objects);

    if(IsKeyPressed(KEY_ESCAPE)) {
        for(size_t i = 0; i < objects->size; ++i) {
            objects->items[i].as.selectable.is_selected = false;
        }
        return;
    }

    if((IsKeyDown(KEY_LEFT_CONTROL) || IsKeyDown(KEY_RIGHT_CONTROL)) && IsKeyPressed(KEY_A)) {
        for(size_t i = 0; i < objects->size; ++i) {
            objects->items[i].as.selectable.is_selected = true;
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
                objects->items[i].as.selectable.is_selected = !objects->items[i].as.selectable.is_selected;
            }
            else {
                objects->items[i].as.selectable.is_selected = true;
            }
        }
        else if(!is_shift_down) {
            objects->items[i].as.selectable.is_selected = false;
        }
    }
}
