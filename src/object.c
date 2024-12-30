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
    assert(objects);

    static CurveTool curve_tool = {
        .color = BLUE,
        .thickness = 5.0f,
    };
    static LineTool  line_tool  = {
        .color = GREEN,
        .thickness = 1.0f,
    };

    switch(mode) {
    case MODE_DRAW_CURVES: {
        Curve_draw_new(camera, &curve_tool);
        if(curve_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_CURVE,
                .as.curve = curve_tool.new_curve,
            });
            curve_tool.finished = false;
        }
        break;
    }
    case MODE_DRAW_LINES: {
        Line_draw_new(camera, &line_tool);
        if(line_tool.finished) {
            Object_array_push_back(objects, (Object) {
                .kind = OBJECT_KIND_LINE,
                .as.line = line_tool.new_line,
            });
            line_tool.finished = false;
        }
        break;
    }
    case MODE_SELECT:
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
