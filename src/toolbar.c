#include "toolbar.h"
#include "color_palette.h"
#include <assert.h>
#include <stdio.h>


static const int   toolbar_height = 30;
static const float max_brightness_ratio = 0.3f;
static const float height_1 = max_brightness_ratio          * toolbar_height;
static const float height_2 = (1.0f - max_brightness_ratio) * toolbar_height;
static const int   padding = 5;
static const Color base_color = {
    .r = 0,
    .g = 82,
    .b = 172,
    .a = 255
};
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

static void draw_toggle_button(Toolbar *toolbar, Button *button) {
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

static void draw_button(Toolbar *toolbar, Button *button) {
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

    if(CheckCollisionPointRec(GetMousePosition(), rect)) {
        if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
            button->is_down = true;
        }
        if(IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
            button->is_down = false;
            button->is_clicked = true;
        }
    }
    else {
        if(IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
            button->is_down = false;
        }
    }

    if(button->is_down) {
        shaddow.x -= 2;
        shaddow.y -= 2;
        background_color = ColorBrightness(base_color, 0.3f);
    }
    
    DrawRectangleRec(shaddow, BLACK);
    DrawRectangleRec(rect, background_color);
    DrawRectangleLinesEx(rect, 1, BLACK);
    DrawText(button->caption, toolbar->x + padding, 1.5 * padding, font_size, BLACK);

    toolbar->x += width + 10;
}

static void draw_tooltip(const char *text) {
    Vector2 position = Vector2Add(GetMousePosition(), (Vector2){ .x = 20, .y = 20 });
    int width = MeasureText(text, font_size);
    DrawRectangleRec((Rectangle){ .x = position.x, .y = position.y, .width = width + 4, .height = font_size }, WHITE);
    DrawText(text, position.x + 2, position.y, font_size, BLACK);
}

static void draw_thickness_selector(Toolbar *toolbar, int width, float max_thickness, ObjectMaker *object_maker) {
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
    if(new_p > 0) {
        object_maker->thickness = new_p;
    }
    p = object_maker->thickness / max_thickness;

    DrawTriangle((Vector2){ .x = toolbar->x, .y = toolbar_height / 2 },
                 (Vector2){ .x = toolbar->x + width, .y = toolbar_height - padding },
                 (Vector2){ .x = toolbar->x + width, .y = padding },
                 BLACK);

    DrawRectangleRec((Rectangle){ .x = toolbar->x + p * width, .y = padding, .width = 3, .height = toolbar_height - 2 * padding }, WHITE);

    toolbar->x += width + 10;
}

static void draw_color_selector(Toolbar *toolbar, ObjectMaker *object_maker) {
    int side_length = toolbar_height - 2 * padding;
    Rectangle rectangle = { .x = toolbar->x, .y = padding, .width = side_length, .height = side_length };
    DrawRectangleRec(rectangle, object_maker->color);
    DrawRectangleLinesEx(rectangle, 2, BLACK);
    toolbar->x += side_length + 10;

    if(IsMouseButtonPressed(MOUSE_BUTTON_LEFT) && CheckCollisionPointRec(GetMousePosition(), rectangle)) {
        toolbar->color_palette.is_shown = true;
        toolbar->color_palette.rectangle = (Rectangle){
            .x = toolbar->x - 100,
            .y = toolbar_height + 10,
            .width = 200,
            .height = 200,
        };
        toolbar->color_palette.background_color = base_color;
    }

    if(toolbar->color_palette.is_shown) {
        ColorPalette_draw(&toolbar->color_palette, &object_maker->color);

        if(IsMouseButtonPressed(MOUSE_BUTTON_LEFT) &&
           !CheckCollisionPointRec(GetMousePosition(), toolbar->color_palette.rectangle) &&
           !CheckCollisionPointRec(GetMousePosition(), rectangle))
        {
            toolbar->color_palette.is_shown = false;
        }
    }
}

void Toolbar_draw(Toolbar *toolbar, ObjectMaker *object_maker) {
    assert(toolbar);
    assert(object_maker);
    toolbar->x = 10;

    const int width = GetScreenWidth();
    DrawRectangleGradientV(0, 0,        width, height_1, ColorBrightness(base_color, 0.4f), ColorBrightness(base_color, 0.5f));
    DrawRectangleGradientV(0, height_1, width, height_2, ColorBrightness(base_color, 0.5f), base_color);

    Button button_select      = (Button){ .caption = "select",     .is_down = (object_maker->kind == OBJECT_KIND_NONE) };
    Button button_draw_curves = (Button){ .caption = "draw curve", .is_down = (object_maker->kind == OBJECT_KIND_CURVE) };
    Button button_draw_lines  = (Button){ .caption = "draw line",  .is_down = (object_maker->kind == OBJECT_KIND_LINE) };

    draw_toggle_button(toolbar, &button_select);
    if(button_select.is_clicked) {
        object_maker->kind = OBJECT_KIND_NONE;
    }
    draw_toggle_button(toolbar, &button_draw_curves);
    if(button_draw_curves.is_clicked) {
        object_maker->kind = OBJECT_KIND_CURVE;
    }
    draw_toggle_button(toolbar, &button_draw_lines);
    if(button_draw_lines.is_clicked) {
        object_maker->kind = OBJECT_KIND_LINE;
    }

    draw_color_selector(toolbar, object_maker);
    draw_thickness_selector(toolbar, 100, 10.0f, object_maker);

    Button button_insert_picture = (Button){ .caption = "insert image", .is_down = false };
    draw_button(toolbar, &button_insert_picture);
    if(button_insert_picture.is_clicked) {
        toolbar->insert_picture = true;
        button_insert_picture.is_clicked = false;
    }
}
