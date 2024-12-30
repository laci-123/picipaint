#include "raypack.h"
#include "curve.h"
#include "object.h"
#include "mode.h"
#include "selection.h"
/* #include "movement.h" */
#include "toolbar.h"
#include <stdio.h>
#include <stdlib.h>


int main(void) {
    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(800, 450, "PiciPaint");
    SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);
    SetExitKey(KEY_NULL);

    Object_array objects = {0};
    Camera2D camera = { .zoom = 1.0f };
    Mode mode = MODE_DRAW_CURVES;

    while(!WindowShouldClose()) {
        float mouse_wheel = GetMouseWheelMove();
        if(mouse_wheel > 0) {
            camera.zoom *= 1.1f;
        }
        else if(mouse_wheel < 0) {
            camera.zoom /= 1.1f;
        }

        camera.offset = (Vector2){
            .x = GetScreenWidth()  / 2.0f,
            .y = GetScreenHeight() / 2.0f,
        };

        if(IsMouseButtonDown(MOUSE_BUTTON_MIDDLE)) {
            Vector2 mouse_delta = Vector2Scale(GetMouseDelta(), 1.0f / camera.zoom);
            camera.target = Vector2Subtract(camera.target, mouse_delta);
        }

        if(mode == MODE_SELECT) {
            Selection_update(camera, &objects);
            //Movement_update(camera, &curves);
        }
    
        BeginDrawing();
            ClearBackground(BLACK);
            BeginMode2D(camera);
                Object_draw_all(camera, mode, &objects);
            EndMode2D();
            Toolbar_draw(&mode);
        EndDrawing();
    }

    Object_array_free(&objects);
    CloseWindow();

    return 0;
}
