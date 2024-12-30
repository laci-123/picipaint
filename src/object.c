#include "object.h"
#include <assert.h>


bool Object_is_moved(const Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        return object->as.curve.is_moved;
    case OBJECT_KIND_LINE:
        return object->as.line.is_moved;
    default:
        assert(false && "unreachable");
    }
}


void Object_move(Vector2 mouse_delta, Object *object) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        Curve_move(mouse_delta, &object->as.curve);
        break;
    case OBJECT_KIND_LINE:
        Line_move(mouse_delta, &object->as.line);
        break;
    default:
        assert(false && "unreachable");
    }
}


void Object_set_moving(Object *object, bool value) {
    assert(object);
    
    switch (object->kind) {
    case OBJECT_KIND_CURVE:
        object->as.curve.is_moved = value;
        break;
    case OBJECT_KIND_LINE:
        object->as.line.is_moved = value;
        break;
    default:
        assert(false && "unreachable");
    }
}


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


void Object_draw_all(Camera2D camera, Tool *tool, Object_array *objects) {
    assert(objects);

    switch(tool->kind) {
    case TOOL_KIND_CURVE: {
        Curve_draw_new(camera, &tool->as.curve_tool);
        if(tool->as.curve_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_CURVE,
                .as.curve = tool->as.curve_tool.new_curve,
            });
            tool->as.curve_tool.finished = false;
        }
        break;
    }
    case TOOL_KIND_LINE: {
        Line_draw_new(camera, &tool->as.line_tool);
        if(tool->as.line_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_LINE,
                .as.line = tool->as.line_tool.new_line,
            });
            tool->as.line_tool.finished = false;
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
        default:
            assert(false && "unreachable");
        }
    }
}
