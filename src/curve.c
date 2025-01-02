#include "curve.h"
#include "toolbar.h"
#include <assert.h>


static Rectangle Curve_get_selection_rect(const Curve *curve) {
    return (Rectangle){
        .x = curve->min_x - curve->thickness,
        .y = curve->min_y - curve->thickness,
        .width = curve->max_x - curve->min_x + 2*curve->thickness,
        .height = curve->max_y - curve->min_y + 2*curve->thickness,
    };
}

void Curve_draw(const Curve *curve) {
    if(curve->base.is_selected) {
        Rectangle rect = Curve_get_selection_rect(curve);
        DrawRectangleLinesEx(rect, 1.0f, WHITE);
    }

    if(curve->points.size == 1) {
        DrawCircleV(curve->points.items[0], curve->thickness * 0.6f, curve->color);
    }
    else {
        for(size_t i = 0; i < curve->points.size; ++i) {
            if(i > 0) {
                Vector2 begin = curve->points.items[i - 1];
                Vector2 end   = curve->points.items[i];
                DrawLineEx(begin, end, curve->thickness, curve->color);
            }
        }
    }
}

void Curve_draw_new(Camera2D camera, CurveTool *tool) {
    assert(tool);

    if(tool->pen_is_down) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            const Vector2 *last_point = Vector2_array_last_const(&tool->new_curve.points);
            if(!(last_point && Vector2Equals(*last_point, mouse_pos))) {
                if(mouse_pos.x < tool->new_curve.min_x) {
                    tool->new_curve.min_x = mouse_pos.x;
                }
                if(mouse_pos.x > tool->new_curve.max_x) {
                    tool->new_curve.max_x = mouse_pos.x;
                }
                if(mouse_pos.y < tool->new_curve.min_y) {
                    tool->new_curve.min_y = mouse_pos.y;
                }
                if(mouse_pos.y > tool->new_curve.max_y) {
                    tool->new_curve.max_y = mouse_pos.y;
                }
                Vector2_array_push_back(&tool->new_curve.points, mouse_pos);
            }
        }
        else {
            tool->pen_is_down = false;
            tool->finished = true;
        }
        Curve_draw(&tool->new_curve);
    }
    else {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT) && !Toolbar_check_collision_point(GetMousePosition())) {
            tool->pen_is_down = true;

            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            tool->new_curve = (Curve){
                    .points = {0},
                    .color = tool->color,
                    .thickness = tool->thickness,
                    .min_x = mouse_pos.x,
                    .max_x = mouse_pos.x,
                    .min_y = mouse_pos.y,
                    .max_y = mouse_pos.y,
            };
        }
    }
}

bool Curve_is_under_mouse(Vector2 mouse_pos, const Curve *curve) {
    assert(curve);

    Rectangle rect = Curve_get_selection_rect(curve);
    if(CheckCollisionPointRec(mouse_pos, rect)) {
        for(size_t i = 0; i < curve->points.size; ++i) {
            if(CheckCollisionPointCircle(mouse_pos, curve->points.items[i], 2 * curve->thickness)) {
                return true;
            }
        }
    }

    return false;
}

void Curve_move(Vector2 mouse_delta, Curve *curve) {
    assert(curve);
  
    for(size_t i = 0; i < curve->points.size; ++i) {
        curve->points.items[i] = Vector2Add(curve->points.items[i], mouse_delta);
    }

    curve->min_x += mouse_delta.x;
    curve->min_y += mouse_delta.y;
    curve->max_x += mouse_delta.x;
    curve->max_y += mouse_delta.y;
}
