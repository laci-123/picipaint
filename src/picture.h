#ifndef PICTURE_INCLUDED_
#define PICTURE_INCLUDED_

#include "raypack.h"
#include "selectable.h"

typedef struct {
    Selectable base;
    Image original_image;
    Texture2D texture;
    Vector2 top_left;
} Picture;

void Picture_draw(const Picture *picture);
bool Picture_is_under_mouse(Vector2 mouse_pos, const Picture *picture);
void Picture_move(Vector2 mouse_delta, Picture *picture);
Rectangle Picture_bounding_rec(const Picture *picture);
void Picture_resize(Rectangle new_size, Picture *picture);

#endif //PICTURE_INCLUDED_
