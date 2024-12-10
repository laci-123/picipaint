#ifndef LINE_INCLUDED_
#define LINE_INCLUDED_

#include "raypack.h"
#include "mode.h"

typedef struct {
  Vector2 start;
  Vector2 end;
  Color color;
  float thickness;
} Line;

#define ELEM_TYPE Line
#include "array.h"
#undef ELEM_TYPE

void Line_draw_all(Camera2D camera, Mode mode, Line_array *liness);
bool Line_is_under_mouse(Vector2 mouse_pos, const Line *line);
void Line_move(Vector2 mouse_delta, Line *line);

#endif //LINE_INCLUDED_
