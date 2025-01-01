#ifndef TOOLBAR_INCLUDED_
#define TOOLBAR_INCLUDED_

#include "raypack.h"
#include "tool.h"
#include "color_palette.h"
#include "user_input.h"

typedef struct {
    int x;
    ColorPalette color_palette;
    UserInput *input;
} Toolbar;

void Toolbar_draw(Toolbar *toolbar, Tool *tool);
bool Toolbar_check_collision_point(Vector2 point);

#endif //TOOLBAR_INCLUDED_
