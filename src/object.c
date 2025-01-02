#include "object.h"
#include <assert.h>


void Object_move(Vector2 mouse_delta, Object *object) {
    assert(object);
    
    switch (object->kind) {
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
    default:
        assert(false && "unreachable");
    }
}


void Object_draw_all(Camera2D camera, Tool *tool, Object_array *objects) {
    assert(objects);

    switch(tool->active) {
    case TOOL_KIND_CURVE: {
        Curve_draw_new(camera, &tool->get.curve_tool);
        if(tool->get.curve_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_CURVE,
                .as.curve = tool->get.curve_tool.new_curve,
            });
            tool->get.curve_tool.finished = false;
        }
        break;
    }
    case TOOL_KIND_LINE: {
        Line_draw_new(camera, &tool->get.line_tool);
        if(tool->get.line_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_LINE,
                .as.line = tool->get.line_tool.new_line,
            });
            tool->get.line_tool.finished = false;
        }
        break;
    }
    case TOOL_KIND_SELECT:
        // do nothing
        break;
    default:
        assert(false && "unreachable");
    }

    for(size_t i = 0; i < objects->size; ++i) {
        switch(objects->items[i].kind) {
        case OBJECT_KIND_CURVE:
            Curve_draw(&objects->items[i].as.curve);
            break;
        case OBJECT_KIND_LINE:
            Line_draw(&objects->items[i].as.line);
            break;
        case OBJECT_KIND_PICTURE:
            Picture_draw(&objects->items[i].as.picture);
            break;
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
    case OBJECT_KIND_LINE:
        // do nothing
        break;
    case OBJECT_KIND_PICTURE:
        UnloadTexture(object->as.picture.texture);
        break;
    default:
        assert(false && "unreachable");
    }
}
