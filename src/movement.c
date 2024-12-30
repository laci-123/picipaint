#include "movement.h"
#include "assert.h"


void Movement_update(Camera2D camera, Object_array *objects) {
    assert(objects);

    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
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
