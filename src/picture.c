#include "picture.h"


void Picture_draw(const Picture *picture) {
    DrawTextureV(picture->texture, picture->top_left, WHITE);
}

bool Picture_is_under_mouse(Vector2 mouse_pos, const Picture *picture) {
    return CheckCollisionPointRec(mouse_pos, (Rectangle){
            .x = picture->top_left.x,
            .y = picture->top_left.y,
            .width = picture->texture.width,
            .height = picture->texture.height,
        });
}

void Picture_move(Vector2 mouse_delta, Picture *picture) {
    picture->top_left = Vector2Add(picture->top_left, mouse_delta);
}
