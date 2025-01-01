#ifndef CURVE_INCLUDED_
#define CURVE_INCLUDED_

#include "raypack.h"
#include "user_input.h"
#define ELEM_TYPE Vector2
#include "array.h"
#undef ELEM_TYPE

typedef struct {
    Vector2_array points;
    Color color;
    float thickness;
    float min_x;
    float max_x;
    float min_y;
    float max_y;
    bool is_selected;
    bool is_moved;
} Curve;

#define ELEM_TYPE Curve
#include "array.h"
#undef ELEM_TYPE

typedef struct {
    Color color;
    float thickness;
    Curve new_curve;
    bool pen_is_down;
    bool finished;
    UserInput *input;
} CurveTool;

void Curve_draw(const Curve *curve);
void Curve_draw_new(Camera2D camera, CurveTool *tool);
bool Curve_is_under_mouse(Vector2 mouse_pos, const Curve *curve);
void Curve_move(Vector2 mouse_delta, Curve *curve);

#endif //CURVE_INCLUDED_
