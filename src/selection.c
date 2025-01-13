#include "selection.h"
#include "assert.h"


typedef struct {
    Vector2 top_left_corner;
    Vector2 top_right_corner;
    Vector2 bottom_left_corner;
    Vector2 bottom_right_corner;
    Rectangle left_side;
    Rectangle right_side;
    Rectangle top_side;
    Rectangle bottom_side;
} RectangleParts;

static RectangleParts parts_of_rectangle(Rectangle rec, int side_thickness) {
    return (RectangleParts){
        .top_left_corner.x = rec.x,
        .top_left_corner.y = rec.y,
        .top_right_corner.x = rec.x + rec.width,
        .top_right_corner.y = rec.y,
        .bottom_left_corner.x = rec.x,
        .bottom_left_corner.y = rec.y + rec.height,
        .bottom_right_corner.x = rec.x + rec.width,
        .bottom_right_corner.y = rec.y + rec.height,
        .left_side.x = rec.x - side_thickness / 2,
        .left_side.y = rec.y,
        .left_side.width = side_thickness,
        .left_side.height = rec.height,
        .right_side.x = rec.x + rec.width - side_thickness / 2,
        .right_side.y = rec.y,
        .right_side.width = side_thickness,
        .right_side.height = rec.height,
        .top_side.x = rec.x,
        .top_side.y = rec.y - side_thickness / 2,
        .top_side.width = rec.width,
        .top_side.height = side_thickness,
        .bottom_side.x = rec.x,
        .bottom_side.y = rec.y + rec.height - side_thickness / 2,
        .bottom_side.width = rec.width,
        .bottom_side.height = side_thickness,
    };
}


static void update_resizing(Camera2D camera, Object_array *objects) {
    Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);

    bool default_cursor = true;
    for(size_t i = 0; i < objects->size; ++i) {
        if(objects->items[i].as.selectable.is_selected) {
            RectangleParts rparts = parts_of_rectangle(Object_bounding_rec(&objects->items[i]), 10);

            if(CheckCollisionPointCircle(mouse_pos, rparts.top_left_corner, 10) || CheckCollisionPointCircle(mouse_pos, rparts.bottom_right_corner, 10)) {
                SetMouseCursor(MOUSE_CURSOR_RESIZE_NWSE);
                default_cursor = false;
            }
            else if(CheckCollisionPointCircle(mouse_pos, rparts.top_right_corner, 10) || CheckCollisionPointCircle(mouse_pos, rparts.bottom_left_corner, 10)) {
                SetMouseCursor(MOUSE_CURSOR_RESIZE_NESW);
                default_cursor = false;
            }
            else if(CheckCollisionPointRec(mouse_pos, rparts.top_side) || CheckCollisionPointRec(mouse_pos, rparts.bottom_side)) {
                SetMouseCursor(MOUSE_CURSOR_RESIZE_NS);
                default_cursor = false;
            }
            else if(CheckCollisionPointRec(mouse_pos, rparts.left_side) || CheckCollisionPointRec(mouse_pos, rparts.right_side)) {
                SetMouseCursor(MOUSE_CURSOR_RESIZE_EW);
                default_cursor = false;
            }
        }
    }
    if(default_cursor) {
        SetMouseCursor(MOUSE_CURSOR_DEFAULT);
    }
}


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
    update_resizing(camera, objects);
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
