#include "object.h"
#include <assert.h>


Rectangle Object_bounding_rec(const Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        return Curve_bounding_rec(&object->as.curve);
    case OBJECT_KIND_LINE:
        return Line_bounding_rec(&object->as.line);
    case OBJECT_KIND_PICTURE:
        return Picture_bounding_rec(&object->as.picture);
    case OBJECT_KIND_NONE:
    default:
        assert(false && "unreachable");
    }
}


void Object_move(Vector2 mouse_delta, Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_NONE:
        // do nothing
        break;
    case OBJECT_KIND_CURVE:
        Curve_move(mouse_delta, &object->as.curve);
        break;
    case OBJECT_KIND_LINE:
        Line_move(mouse_delta, &object->as.line);
        break;
    case OBJECT_KIND_PICTURE:
        Picture_move(mouse_delta, &object->as.picture);
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
    case OBJECT_KIND_PICTURE:
        return Picture_is_under_mouse(mouse_pos, &object->as.picture);
    case OBJECT_KIND_NONE:
    default:
        assert(false && "unreachable");
    }
}


void Object_draw_all(Camera2D camera, ObjectMaker *maker) {
    assert(maker);

    if(!maker->toolbar_is_focused || maker->pen_is_down) {
        switch(maker->kind) {
        case OBJECT_KIND_CURVE:
            Curve_draw_new(camera, maker);
            break;
        case OBJECT_KIND_LINE:
            Line_draw_new(camera, maker);
            break;
        case OBJECT_KIND_NONE:
        case OBJECT_KIND_PICTURE:
            // do nothing
            break;
        default:
            assert(false && "unreachable");
        }
    }

    for(size_t i = 0; i < maker->objects.size; ++i) {
        switch(maker->objects.items[i].kind) {
        case OBJECT_KIND_CURVE:
            Curve_draw(&maker->objects.items[i].as.curve);
            break;
        case OBJECT_KIND_LINE:
            Line_draw(&maker->objects.items[i].as.line);
            break;
        case OBJECT_KIND_PICTURE:
            Picture_draw(&maker->objects.items[i].as.picture);
            break;
        case OBJECT_KIND_NONE:
        default:
            assert(false && "unreachable");
        }
    }
}


void Object_free(Object *object) {
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        Vector2_array_free(&object->as.curve.points);
        break;
    case OBJECT_KIND_PICTURE:
        UnloadTexture(object->as.picture.texture);
        break;
    case OBJECT_KIND_NONE:
    case OBJECT_KIND_LINE:
        // do nothing
        break;
    default:
        assert(false && "unreachable");
    }
}

void Object_resize(Rectangle new_size, Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_NONE:
        // do nothing
        break;
    case OBJECT_KIND_CURVE:
        Curve_resize(new_size, &object->as.curve);
        break;
    case OBJECT_KIND_LINE:
        Line_resize(new_size, &object->as.line);
        break;
    case OBJECT_KIND_PICTURE:
        Picture_resize(new_size, &object->as.picture);
        break;
    default:
        assert(false && "unreachable");
    }
}
