#ifndef CURVE_INCLUDED_
#define CURVE_INCLUDED_

#include "raypack.h"
#include "mode.h"
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

void draw_curves(Camera2D camera, Mode mode, Curve_array *curves);
bool curve_is_mouse_over(Vector2 mouse_pos, const Curve *curve);
void move_curve(Vector2 mouse_delta, Curve *curve);

#endif //CURVE_INCLUDED_
