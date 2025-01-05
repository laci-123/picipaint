#include "raypack.h"
#include "curve.h"
#include "object.h"
#include "picture_loader.h"
#include "selection.h"
#include "toolbar.h"
#include "tool.h"
#include <stdio.h>
#include <stdlib.h>

#include <nfd.h>


int main(void) {
    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(800, 450, "PiciPaint");
    SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);
    SetExitKey(KEY_NULL);

    NFD_Init();

    nfdu8char_t *out_path;
    nfdu8filteritem_t filters[2] = { { "Source code", "c,cpp,cc" }, { "Headers", "h,hpp" } };
    nfdopendialogu8args_t args = {0};
    args.filterList = filters;
    args.filterCount = 2;
    nfdresult_t result = NFD_OpenDialogU8_With(&out_path, &args); 
    if(result == NFD_OKAY) {
        TraceLog(LOG_INFO, "success: %s", out_path);
        NFD_FreePathU8(out_path);
    }
    else if(result == NFD_CANCEL) {
        TraceLog(LOG_INFO, "canceled");
    }
    else {
        TraceLog(LOG_INFO, "error: %s", NFD_GetError());
    }

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

        load_dropped_pictures(&objects, camera);
    
        BeginDrawing();
            ClearBackground(BLACK);
            BeginMode2D(camera);
                Object_draw_all(camera, &tool, &objects);
            EndMode2D();
            Toolbar_draw(&toolbar, &tool);
        EndDrawing();
    }

    NFD_Quit();

    for(size_t i = 0; i < objects.size; ++i) {
        Object_free(&objects.items[i]);
    }
    Object_array_free(&objects);
    CloseWindow();

    return 0;
}
