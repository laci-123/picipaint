#ifndef TOOLBAR_INCLUDED_
#define TOOLBAR_INCLUDED_

#include "raypack.h"
#include "mode.h"

void Toolbar_draw(Mode *mode);
bool Toolbar_check_collision_point(Vector2 point);

#endif //TOOLBAR_INCLUDED_
