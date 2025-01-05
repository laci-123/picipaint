#ifndef TOOLBAR_INCLUDED_
#define TOOLBAR_INCLUDED_

#include "raypack.h"
#include "tool.h"
#include "color_palette.h"

typedef struct {
    int x;
    ColorPalette color_palette;
    bool insert_picture;
} Toolbar;

void Toolbar_draw(Toolbar *toolbar, Tool *tool);
bool Toolbar_check_collision_point(Vector2 point);

#endif //TOOLBAR_INCLUDED_
