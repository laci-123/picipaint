#ifndef OBJECT_INCLUDED_
#define OBJECT_INCLUDED_

#include "curve.h"
#include "line.h"
#include "raypack.h"


typedef enum {
    OBJECT_KIND_CURVE,
    OBJECT_KIND_LINE,
} ObjectKind;


typedef struct {
    union {
        Curve curve;
        Line line;
    } as;
    ObjectKind kind;
} Object;


bool Object_is_selected(const Object *object);
void Object_set_selection(Object *object, bool value);
bool Object_is_under_mouse(Vector2 mouse_pos, const Object *object);

#define ELEM_TYPE Object
#include "array.h"
#undef ELEM_TYPE

void Object_draw_all(Camera2D camera, Mode mode, Object_array *objects);

#endif //OBJECT_INCLUDED_
