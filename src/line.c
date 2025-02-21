#include "line.h"
#include "object.h"
#include <assert.h>


void Line_draw_new(Camera2D camera, ObjectMaker *maker) {
    assert(maker);
    
    if(maker->pen_is_down) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);

            Object *new_object = &maker->objects.items[maker->objects.size - 1];
            assert(new_object->kind == OBJECT_KIND_LINE);
            Line *new_line = &new_object->as.line;

            new_line->end = mouse_pos;
        }
        else {
            maker->pen_is_down = false;
        }
    }
    else {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            maker->pen_is_down = true;
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            Line new_line = {
                .start = mouse_pos,
                .end = mouse_pos,
                .color = maker->color,
                .thickness = maker->thickness,
            };
            Object_array_push_back(&maker->objects, (Object){ .kind = OBJECT_KIND_LINE, .as.line = new_line });
        }
    }
}

void Line_draw(const Line *line) {
    assert(line);
    
    DrawLineEx(line->start, line->end, line->thickness, line->color);
}

bool Line_is_under_mouse(Vector2 mouse_pos, const Line *line) {
    assert(line);

    float d1 = Vector2Distance(line->start, mouse_pos);
    float d2 = Vector2Distance(mouse_pos,   line->end);
    float d3 = Vector2Distance(line->start, line->end);

    return d1 + d2 - d3 < 10.0f;
}

void Line_move(Vector2 mouse_delta, Line *line) {
    assert(line);

    line->start = Vector2Add(line->start, mouse_delta);
    line->end = Vector2Add(line->end,   mouse_delta);
}

Rectangle Line_bounding_rec(const Line *line) {
    assert(line);
    
    return (Rectangle){
        .x = fmin(line->start.x, line->end.x),
        .y = fmin(line->start.y, line->end.y),
        .width  = fabs(line->end.x - line->start.x),
        .height = fabs(line->end.y - line->start.y),
    };
}

void Line_resize(Rectangle new_size, Line *line) {
    assert(line);


    float start_x = line->start.x < line->end.x ? new_size.x : new_size.x + new_size.width; 
    float start_y = line->start.y < line->end.y ? new_size.y : new_size.y + new_size.height; 
    float end_x   = line->start.x > line->end.x ? new_size.x : new_size.x + new_size.width; 
    float end_y   = line->start.y > line->end.y ? new_size.y : new_size.y + new_size.height; 

    line->start.x = start_x;
    line->start.y = start_y;
    line->end.x   = end_x; 
    line->end.y   = end_y; 
}
