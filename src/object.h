#ifndef OBJECT_INCLUDED_
#define OBJECT_INCLUDED_

#include "curve.h"
#include "selectable.h"
#include "line.h"
#include "picture.h"
#include "tool.h"
#include "raypack.h"


typedef enum {
    OBJECT_KIND_CURVE,
    OBJECT_KIND_LINE,
    OBJECT_KIND_PICTURE,
} ObjectKind;


typedef struct {
    union {
        Selectable selectable;
        Curve curve;
        Line line;
        Picture picture;
    } as;
    ObjectKind kind;
} Object;


void Object_move(Vector2 mouse_delta, Object *object);
bool Object_is_under_mouse(Vector2 mouse_pos, const Object *object);
void Object_free(Object *object);

#define ELEM_TYPE Object
#include "array.h"
#undef ELEM_TYPE

void Object_draw_all(Camera2D camera, Tool *tool, Object_array *objects);

#endif //OBJECT_INCLUDED_
