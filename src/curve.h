#ifndef CURVE_INCLUDED_
#define CURVE_INCLUDED_

#include "raylib.h"
#define ELEM_TYPE Vector2
#include "array.h"
#undef ELEM_TYPE

typedef struct {
  Vector2_array points;
  Color color;
  float thickness;
} Curve;

#define ELEM_TYPE Curve
#include "array.h"
#undef ELEM_TYPE

void draw_curves(Curve_array *curves);

#endif //CURVE_INCLUDED_
