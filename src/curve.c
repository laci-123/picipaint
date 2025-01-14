#include "curve.h"
#include "object.h"
#include <assert.h>


Rectangle Curve_bounding_rec(const Curve *curve) {
    return (Rectangle){
        .x = curve->min_x - curve->thickness,
        .y = curve->min_y - curve->thickness,
        .width = curve->max_x - curve->min_x + 2*curve->thickness,
        .height = curve->max_y - curve->min_y + 2*curve->thickness,
    };
}

void Curve_draw(const Curve *curve) {
    if(curve->base.is_selected) {
        DrawRectangleLinesEx(Curve_bounding_rec(curve), 1.0f, WHITE);
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

void Curve_draw_new(Camera2D camera, ObjectMaker *maker) {
    assert(maker);

    if(maker->pen_is_down) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);

            Object *new_object = &maker->objects.items[maker->objects.size - 1];
            assert(new_object->kind == OBJECT_KIND_CURVE);
            Curve *new_curve = &new_object->as.curve;
            const Vector2 *last_point = Vector2_array_last_const(&new_curve->points);

            if(!(last_point && Vector2Equals(*last_point, mouse_pos))) {
                if(mouse_pos.x < new_curve->min_x) {
                    new_curve->min_x = mouse_pos.x;
                }
                if(mouse_pos.x > new_curve->max_x) {
                    new_curve->max_x = mouse_pos.x;
                }
                if(mouse_pos.y < new_curve->min_y) {
                    new_curve->min_y = mouse_pos.y;
                }
                if(mouse_pos.y > new_curve->max_y) {
                    new_curve->max_y = mouse_pos.y;
                }
                Vector2_array_push_back(&new_curve->points, mouse_pos);
            }
        }
        else {
            maker->pen_is_down = false;
        }
    }
    else {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            maker->pen_is_down = true;

            Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
            Curve new_curve = {
                    .points = {0},
                    .color = maker->color,
                    .thickness = maker->thickness,
                    .min_x = mouse_pos.x,
                    .max_x = mouse_pos.x,
                    .min_y = mouse_pos.y,
                    .max_y = mouse_pos.y,
            };
            Object_array_push_back(&maker->objects, (Object){ .kind = OBJECT_KIND_CURVE, .as.curve = new_curve });
        }
    }
}

bool Curve_is_under_mouse(Vector2 mouse_pos, const Curve *curve) {
    assert(curve);

    Rectangle rect = Curve_bounding_rec(curve);
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

void Curve_resize(Rectangle new_size, Curve *curve) {
    assert(curve);
    
    float scale = (float)new_size.width / (float)(curve->max_x - curve->min_x);
    for(size_t i = 0; i < curve->points.size; ++i) {
        Vector2Scale(curve->points.items[i], scale);
    }
    
    curve->min_x = new_size.x;
    curve->min_y = new_size.y;
    curve->max_x = new_size.x + new_size.width;
    curve->max_y = new_size.y + new_size.height;
}
