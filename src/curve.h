#ifndef CURVE_INCLUDED_
#define CURVE_INCLUDED_

#include "raypack.h"
#include "object_maker_fwd.h"
#include "selectable.h"
#define ELEM_TYPE Vector2
#include "array.h"
#undef ELEM_TYPE

typedef struct {
    Selectable base;
    Vector2_array points;
    Color color;
    float thickness;
    float min_x;
    float max_x;
    float min_y;
    float max_y;
} Curve;

void Curve_draw(const Curve *curve);
void Curve_draw_new(Camera2D camera, ObjectMaker *maker);
bool Curve_is_under_mouse(Vector2 mouse_pos, const Curve *curve);
void Curve_move(Vector2 mouse_delta, Curve *curve);
Rectangle Curve_bounding_rec(const Curve *curve);

#endif //CURVE_INCLUDED_
