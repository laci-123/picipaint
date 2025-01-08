#ifndef OBJECT_INCLUDED_
#define OBJECT_INCLUDED_

#include "curve.h"
#include "line.h"
#include "picture.h"

#include "selectable.h"
#include "object_maker_fwd.h"
#include "raypack.h"


typedef enum {
    OBJECT_KIND_NONE,
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
void Object_draw_all(Camera2D camera, ObjectMaker *maker);


#define ELEM_TYPE Object
#include "array.h"
#undef ELEM_TYPE


struct ObjectMaker {
    Object_array objects;
    Color color;
    float thickness;
    ObjectKind kind;
    bool pen_is_down;
    bool toolbar_is_focused;
};

#endif //OBJECT_INCLUDED_
