#include "picture.h"


void Picture_draw(const Picture *picture) {
    if(picture->base.is_selected) {
        Rectangle rect = {
            .x = picture->top_left.x - 2,
            .y = picture->top_left.y - 2,
            .width = picture->texture.width + 4,
            .height = picture->texture.height + 4,
        };
        DrawRectangleLinesEx(rect, 1.0f, WHITE);
    }
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
