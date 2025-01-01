#ifndef COLOR_PALETTE_INCLUDED_
#define COLOR_PALETTE_INCLUDED_

#include "raypack.h"
#include "user_input.h"

typedef struct {
    Rectangle rectangle;
    Color background_color;
    bool is_shown;
    bool is_rg_selector_moved;
    bool is_b_selector_moved;
} ColorPalette;

void ColorPalette_draw(ColorPalette *color_palette, Color *color, UserInput *input);

#endif //COLOR_PALETTE_INCLUDED_
