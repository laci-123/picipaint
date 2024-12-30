#include "line.h"
#include "toolbar.h"
#include <assert.h>


static void Line_draw_new(Camera2D camera, Line_array *lines) {
    static bool pen_down = false;
  
    if(pen_down) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            Line *last_line = Line_array_last(lines);
            assert(last_line && "`lines` has at least one element, because when the pen was put down we added one.");
            last_line->end = mouse_pos;
        }
        else {
            pen_down = false;
        }
    }
    else {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT) && !Toolbar_check_collision_point(GetMousePosition())) {
            pen_down = true;
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            Line_array_push_back(lines, (Line){
                    .start = mouse_pos,
                    .end = mouse_pos,
                    .color = WHITE,
                    .thickness = 5.0f,
                });
        }
    }
}

static void Line_draw(const Line *line) {
    DrawLineEx(line->start, line->end, line->thickness, line->color);
}

void Line_draw_all(Camera2D camera, Mode mode, Line_array *lines) {
    assert(lines);

    if(mode == MODE_DRAW_LINES) {
        Line_draw_new(camera, lines);
    }

    /* bool is_delete_pressed = IsKeyPressed(KEY_DELETE); */
    for(size_t i = 0; i < lines->size; ++i) {
        /* if(lines->items[i].is_selected && is_delete_pressed) { */
        /*   Line deleted = Line_array_delete(lines, i--); array_delete messes with the indexes, be careful! */
        /*   Vector2_array_free(&deleted.points); */
        /* } */
        /* else { */
        Line_draw(&lines->items[i]);
        /* } */
    }
    /* Line_array_shrink_to_fit(lines); */
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

    Vector2Add(line->start, mouse_delta);
    Vector2Add(line->end,   mouse_delta);
}
