#include "raypack.h"
#include "curve.h"
#include "object.h"
#include "picture_loader.h"
#include "selection.h"
#include "toolbar.h"
#include "tool.h"
#include "camera.h"
#include <stdio.h>
#include <stdlib.h>


int main(void) {
    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(800, 450, "PiciPaint");
    SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);
    SetExitKey(KEY_NULL);

    Object_array objects = {0};

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
        Camera_update(&camera);

        if(tool.active == TOOL_KIND_SELECT) {
            Selection_update(camera, &objects);
        }

        load_dropped_pictures(&objects, camera);
        if(toolbar.insert_picture) {
            load_picture_using_file_dialog(&objects);
            toolbar.insert_picture = false;
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
