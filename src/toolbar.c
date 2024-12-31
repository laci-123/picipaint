#include "toolbar.h"
#include "assert.h"
#include <stdio.h>


static const int   toolbar_height = 30;
static const float max_brightness_ratio = 0.3f;
static const float height_1 = max_brightness_ratio          * toolbar_height;
static const float height_2 = (1.0f - max_brightness_ratio) * toolbar_height;
static const int   padding = 5;
static const Color base_color = DARKBLUE;
static const int font_size = 14;

bool Toolbar_check_collision_point(Vector2 point) {
    const Rectangle rect = {
        .x = 0,
        .y = 0,
        .width = GetScreenWidth(),
        .height = toolbar_height,
    };
    return CheckCollisionPointRec(point, rect);
}

typedef struct {
    char *caption;
    bool is_down;
    bool is_clicked;
} Button;

static void draw_button(Toolbar *toolbar, Button *button) {
    assert(toolbar);
    assert(button);
    assert(button->caption);
  
    Color background_color = ColorBrightness(base_color, 0.4f);
    const int width = MeasureText(button->caption, font_size) + 2 * padding;
    const Rectangle rect = {
        .x = toolbar->x,
        .y = padding,
        .width = width,
        .height = toolbar_height - 2 * padding,
    };
    Rectangle shaddow = {
        .x = rect.x,
        .y = rect.y,
        .width = rect.width + 2,
        .height = rect.height + 2,
    };
    if(button->is_down) {
        shaddow.x -= 2;
        shaddow.y -= 2;
        background_color = ColorBrightness(base_color, 0.3f);
    }
    if(CheckCollisionPointRec(GetMousePosition(), rect)) {
        if(IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
            button->is_down = true;
            button->is_clicked = true;
        }
    }
    DrawRectangleRec(shaddow, BLACK);
    DrawRectangleRec(rect, background_color);
    DrawRectangleLinesEx(rect, 1, BLACK);
    DrawText(button->caption, toolbar->x + padding, 1.5 * padding, font_size, BLACK);

    toolbar->x += width + 10;
}

static void draw_tooltip(const char *text) {
    assert(text);

    Vector2 position = Vector2Add(GetMousePosition(), (Vector2){ .x = 20, .y = 20 });
    int width = MeasureText(text, font_size);
    DrawRectangleRec((Rectangle){ .x = position.x, .y = position.y, .width = width + 4, .height = font_size }, WHITE);
    DrawText(text, position.x + 2, position.y, font_size, BLACK);
}

static void draw_thickness_selector(Toolbar *toolbar, int width, float max_thickness, Tool *tool) {
    assert(toolbar);
    assert(tool);

    float new_p = -1;
    Vector2 mouse_pos = GetMousePosition();
    if(CheckCollisionPointRec(mouse_pos, (Rectangle){ .x = toolbar->x, .y = padding, .width = width, .height = toolbar_height - 2 * padding })) {
        float value_under_mouse = max_thickness * (float)(mouse_pos.x - toolbar->x) / (float)width;
        if(IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
            new_p = value_under_mouse;
        }
        else {
            char text[32];
            sprintf(text, "thickness: %.1f", value_under_mouse);
            draw_tooltip(text);
        }
    }

    float p;
    bool enabled;
    switch(tool->active) {
    case TOOL_KIND_CURVE:
        if(new_p > 0) {
            tool->get.curve_tool.thickness = new_p;
            p = new_p;
        }
        else{
            p = tool->get.curve_tool.thickness / max_thickness;
        }
        enabled = true;
        break;
    case TOOL_KIND_LINE:
        if(new_p > 0) {
            tool->get.line_tool.thickness = new_p;
            p = new_p;
        }
        else{
            p = tool->get.line_tool.thickness / max_thickness;
        }
        enabled = true;
        break;
    default:
        p = 0.5f;
        enabled = false;
    }

    DrawTriangle((Vector2){ .x = toolbar->x, .y = toolbar_height / 2 },
                 (Vector2){ .x = toolbar->x + width, .y = toolbar_height - padding },
                 (Vector2){ .x = toolbar->x + width, .y = padding },
                 enabled ? BLACK : GRAY);

    DrawRectangleRec((Rectangle){ .x = toolbar->x + p * width, .y = padding, .width = 3, .height = toolbar_height - 2 * padding },
                     enabled ? WHITE : GRAY);

    toolbar->x += width;
}

static void draw_color_selector(Toolbar *toolbar, Tool *tool) {
    int side_length = toolbar_height - 2 * padding;
    Rectangle rectangle = { .x = toolbar->x, .y = padding, .width = side_length, .height = side_length };
    Color color;
    Color border_color;
    switch(tool->active) {
    case TOOL_KIND_CURVE:
        color = tool->get.curve_tool.color;
        border_color = BLACK;
        break;
    case TOOL_KIND_LINE:
        color = tool->get.line_tool.color;
        border_color = BLACK;
        break;
    default:
        color = GRAY;
        border_color = GRAY;
    }
    DrawRectangleRec(rectangle, color);
    DrawRectangleLinesEx(rectangle, 2, border_color);
    toolbar->x += side_length + 10;
}

void Toolbar_draw(Toolbar *toolbar, Tool *tool) {
    toolbar->x = 10;

    const int width = GetScreenWidth();
    DrawRectangleGradientV(0, 0,        width, height_1, ColorBrightness(base_color, 0.4f), ColorBrightness(base_color, 0.5f));
    DrawRectangleGradientV(0, height_1, width, height_2, ColorBrightness(base_color, 0.5f), base_color);

    Button button_select      = (Button){ .caption = "select",     .is_down = (tool->active == TOOL_KIND_SELECT) };
    Button button_draw_curves = (Button){ .caption = "draw curve", .is_down = (tool->active == TOOL_KIND_CURVE) };
    Button button_draw_lines  = (Button){ .caption = "draw line",  .is_down = (tool->active == TOOL_KIND_LINE) };

    draw_button(toolbar, &button_select);
    if(button_select.is_clicked) {
        tool->active = TOOL_KIND_SELECT;
    }
    draw_button(toolbar, &button_draw_curves);
    if(button_draw_curves.is_clicked) {
        tool->active = TOOL_KIND_CURVE;
    }
    draw_button(toolbar, &button_draw_lines);
    if(button_draw_lines.is_clicked) {
        tool->active = TOOL_KIND_LINE;
    }

    draw_color_selector(toolbar, tool);
    draw_thickness_selector(toolbar, 100, 10.0f, tool);
}
