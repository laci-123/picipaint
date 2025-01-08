#include "camera.h"
#include <assert.h>


void Camera_update(Camera2D *camera) {
    assert(camera);
    
    float mouse_wheel = GetMouseWheelMove();
    if(mouse_wheel > 0) {
        camera->zoom *= 1.1f;
    }
    else if(mouse_wheel < 0) {
        camera->zoom /= 1.1f;
    }

    camera->offset = (Vector2){
        .x = GetScreenWidth()  / 2.0f,
        .y = GetScreenHeight() / 2.0f,
    };

    if(IsMouseButtonDown(MOUSE_BUTTON_MIDDLE)) {
        Vector2 mouse_delta = Vector2Scale(GetMouseDelta(), 1.0f / camera->zoom);
        camera->target = Vector2Subtract(camera->target, mouse_delta);
    }
}
