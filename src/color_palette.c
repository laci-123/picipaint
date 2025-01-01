#include "color_palette.h"
#include <assert.h>


void ColorPalette_draw(ColorPalette *cp, Color *color, UserInput *input) {
    assert(cp);
    assert(color);

    DrawRectangleRounded(cp->rectangle, 0.1f, 10, cp->background_color);
    
    Rectangle rg_rectangle = {
        .x = cp->rectangle.x + 10,
        .y = cp->rectangle.y + 10,
        .width = cp->rectangle.width - 20,
        .height = cp->rectangle.height - 50,
    };

    for(int i = 0; i < rg_rectangle.width; ++i) {
        for(int j = 0; j < rg_rectangle.height; ++j) {
            int x = 255.0f * ((float)i / (float)rg_rectangle.width);
            int y = 255.0f * ((float)j / (float)rg_rectangle.height);
            DrawPixel(rg_rectangle.x + i, rg_rectangle.y + j, (Color){ .r = x, .g = y, .b = color->b, .a = 255 });
        }
    }
    DrawRectangleLinesEx(rg_rectangle, 2, BLACK);

    Rectangle b_rectangle = {
        .x = rg_rectangle.x,
        .y = rg_rectangle.y + rg_rectangle.height + 10,
        .width = rg_rectangle.width,
        .height = 20,
    };

    for(int i = 0; i < b_rectangle.width; ++i) {
        for(int j = 0; j < b_rectangle.height; ++j) {
            int x = 255.0f * ((float)i / (float)b_rectangle.width);
            DrawPixel(b_rectangle.x + i, b_rectangle.y + j, (Color){ .r = color->r, .g = color->g, .b = x, .a = 255 });
        }
    }
    DrawRectangleLinesEx(b_rectangle, 2, BLACK);

    Vector2 color_pos_rg = {
        .x = rg_rectangle.x + (float)rg_rectangle.width  * (float)color->r / 255.0f,
        .y = rg_rectangle.y + (float)rg_rectangle.height * (float)color->g / 255.0f,
    };
    int color_pos_b = b_rectangle.x + (float)b_rectangle.width * (float)color->b / 255.0f;
    Rectangle blue_selector = { .x = color_pos_b, .y = b_rectangle.y, .width = 3, .height = b_rectangle.height };
    DrawRing(color_pos_rg, 4.0f, 6.0f, 0.0f, 360.0f, 20, BLACK);
    DrawRectangleRec(blue_selector, BLACK);

    if(CheckCollisionPointCircle(GetMousePosition(), color_pos_rg, 6.0f)) {
        cp->is_rg_selector_moved = true;
    }
    if(cp->is_rg_selector_moved) {
        if(is_mouse_button_down(input, MOUSE_BUTTON_LEFT)) {
            Vector2 new_pos_rg = GetMousePosition(); 
            if(CheckCollisionPointRec(new_pos_rg, rg_rectangle)) {
                color->r = 255.0f * (float)(new_pos_rg.x - rg_rectangle.x) / (float)rg_rectangle.width;
                color->g = 255.0f * (float)(new_pos_rg.y - rg_rectangle.y) / (float)rg_rectangle.height;
            }
        }
        else {
            cp->is_rg_selector_moved = false;
        }
    }

    if(CheckCollisionPointRec(GetMousePosition(), blue_selector)) {
        cp->is_b_selector_moved = true;
    }
    if(cp->is_b_selector_moved) {
        if(is_mouse_button_down(input, MOUSE_BUTTON_LEFT)) {
            int new_pos_b = GetMousePosition().x;
            if(b_rectangle.x <= new_pos_b && new_pos_b < b_rectangle.x + b_rectangle.width) {
                color->b = 255.0f * (float)(new_pos_b - b_rectangle.x) / (float)b_rectangle.width;
            }
        }
        else {
            cp->is_b_selector_moved = false;
        }
    }
}
