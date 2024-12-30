#include "object.h"
#include <assert.h>


bool Object_is_selected(const Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        return object->as.curve.is_selected;
    case OBJECT_KIND_LINE:
        return object->as.line.is_selected;
    default:
        assert(false && "unreachable");
    }
}


void Object_set_selection(Object *object, bool value) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        object->as.curve.is_selected = value;
        break;
    case OBJECT_KIND_LINE:
        object->as.line.is_selected = value;
        break;
    default:
        assert(false && "unreachable");
    }
}


bool Object_is_under_mouse(Vector2 mouse_pos, const Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        return Curve_is_under_mouse(mouse_pos, &object->as.curve);
    case OBJECT_KIND_LINE:
        return Line_is_under_mouse(mouse_pos, &object->as.line);
    default:
        assert(false && "unreachable");
    }
}


void Object_draw_all(Camera2D camera, Mode mode, Object_array *objects) {
    Curve_array curves = {0};
    Line_array lines = {0};

    for(size_t i = 0; i < objects->size; ++i) {
        switch (objects->items[i].kind) {
        case OBJECT_KIND_CURVE:
            Curve_array_push_back(&curves, objects->items[i].as.curve);
            break;
        case OBJECT_KIND_LINE:
            Line_array_push_back(&lines, objects->items[i].as.line);
            break;
        default:
            assert(false && "unreachable");
        }
    }

    Curve_draw_all(camera, mode, &curves);
    Line_draw_all(camera, mode, &lines);

    objects->size = 0;
    for(size_t i = 0; i < curves.size; ++i) {
        Object object;
        object.kind = OBJECT_KIND_CURVE;
        object.as.curve = curves.items[i];
        Object_array_push_back(objects, object);
    }
    for(size_t i = 0; i < lines.size; ++i) {
        Object object;
        object.kind = OBJECT_KIND_LINE;
        object.as.line = lines.items[i];
        Object_array_push_back(objects, object);
    }

    Line_array_free(&lines);
    Curve_array_free(&curves);
    Object_array_shrink_to_fit(objects);
}
