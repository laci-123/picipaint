#include "picture.h"
#include <assert.h>


Rectangle Picture_bounding_rec(const Picture *picture) {
    assert(picture);
    
    return (Rectangle){
        .x = picture->top_left.x,
        .y = picture->top_left.y,
        .width = picture->texture.width,
        .height = picture->texture.height,
    };
}


void Picture_draw(const Picture *picture) {
    assert(picture);
    
    DrawTextureV(picture->texture, picture->top_left, WHITE);
}

bool Picture_is_under_mouse(Vector2 mouse_pos, const Picture *picture) {
    assert(picture);
    
    return CheckCollisionPointRec(mouse_pos, (Rectangle){
            .x = picture->top_left.x,
            .y = picture->top_left.y,
            .width = picture->texture.width,
            .height = picture->texture.height,
        });
}

void Picture_move(Vector2 mouse_delta, Picture *picture) {
    assert(picture);

    picture->top_left = Vector2Add(picture->top_left, mouse_delta);
}

void Picture_resize(Rectangle new_size, Picture *picture) {
    assert(picture);
        
    picture->top_left.x = new_size.x;
    picture->top_left.y = new_size.y;

    Image image = ImageCopy(picture->original_image);
    ImageResize(&image, new_size.width, new_size.height);
    UnloadTexture(picture->texture);
    picture->texture = LoadTextureFromImage(image);
    UnloadImage(image);
}
