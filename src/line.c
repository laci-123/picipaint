#include "line.h"
#include "toolbar.h"
#include <assert.h>


void Line_draw_new(Camera2D camera, LineTool *tool) {
    if(tool->pen_is_down) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            tool->new_line.end = mouse_pos;
        }
        else {
            tool->pen_is_down = false;
            tool->finished = true;
        }
        Line_draw(&tool->new_line);
    }
    else {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT) && !Toolbar_check_collision_point(GetMousePosition())) {
            tool->pen_is_down = true;
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            tool->new_line = (Line){
                .start = mouse_pos,
                .end = mouse_pos,
                .color = tool->color,
                .thickness = tool->thickness,
            };
        }
    }
}

void Line_draw(const Line *line) {
    if(line->is_selected) {
        Rectangle rect = {
            .x = fmin(line->start.x, line->end.x) - line->thickness,
            .y = fmin(line->start.y, line->end.y) - line->thickness,
            .width  = fabs(line->end.x - line->start.x) + 2 * line->thickness,
            .height = fabs(line->end.y - line->start.y) + 2 * line->thickness,
        };
        DrawRectangleLinesEx(rect, 1.0f, WHITE);
    }
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
