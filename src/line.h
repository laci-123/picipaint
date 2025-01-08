#ifndef LINE_INCLUDED_
#define LINE_INCLUDED_

#include "raypack.h"
#include "selectable.h"
#include "object_maker_fwd.h"

typedef struct {
    Selectable base;
    Vector2 start;
    Vector2 end;
    Color color;
    float thickness;
} Line;

void Line_draw_new(Camera2D camera, ObjectMaker *maker);
void Line_draw(const Line *line);
bool Line_is_under_mouse(Vector2 mouse_pos, const Line *line);
void Line_move(Vector2 mouse_delta, Line *line);

#endif //LINE_INCLUDED_
