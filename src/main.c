#include "raypack.h"
#include "curve.h"
#include "object.h"
#include "selection.h"
#include "toolbar.h"
#include "tool.h"
#include <stdio.h>
#include <stdlib.h>

#include "picture.h"


int main(void) {
    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(800, 450, "PiciPaint");
    SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);
    SetExitKey(KEY_NULL);

    Object_array objects = {0};

    Image test_image = GenImageColor(100, 100, RED);
    Picture test_picture = {
        .texture = LoadTextureFromImage(test_image),
        .top_left = (Vector2){ .x = 10, .y = 50 },
    };
    UnloadImage(test_image);
    Object_array_push_back(&objects, (Object){.as.picture = test_picture, .kind = OBJECT_KIND_PICTURE});

    Camera2D camera = { .zoom = 1.0f };
    Tool tool = {
        .active = TOOL_KIND_CURVE,
        .get.curve_tool = (CurveTool) {
            .color = BLUE,
            .thickness = 5.0f,
        },
        .get.line_tool = (LineTool) {
            .color = GREEN,
            .thickness = 3.0f,
        },
    };
    Toolbar toolbar = {0};

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

        if(tool.active == TOOL_KIND_SELECT) {
            Selection_update(camera, &objects);
        }
    
        BeginDrawing();
            ClearBackground(BLACK);
            BeginMode2D(camera);
                Object_draw_all(camera, &tool, &objects);
            EndMode2D();
            Toolbar_draw(&toolbar, &tool);
        EndDrawing();
    }

    for(size_t i = 0; i < objects.size; ++i) {
        Object_free(&objects.items[i]);
    }
    Object_array_free(&objects);
    CloseWindow();

    return 0;
}
