#include "raypack.h"
#include "object.h"
#include "picture_loader.h"
#include "selection.h"
#include "toolbar.h"
#include "camera.h"
#include <stdio.h>
#include <stdlib.h>


int main(void) {
    SetConfigFlags(FLAG_MSAA_4X_HINT);
    InitWindow(800, 450, "PiciPaint");
    SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);
    SetExitKey(KEY_NULL);

    ObjectMaker object_maker = {
        .color = BLUE,
        .thickness = 3.0f,
        .kind = OBJECT_KIND_CURVE,
    };

    Camera2D camera = { .zoom = 1.0f };
    Toolbar toolbar = {0};

    while(!WindowShouldClose()) {
        Camera_update(&camera);

        if(object_maker.kind == OBJECT_KIND_NONE) {
            Selection_update(camera, &object_maker.objects);
        }

        load_dropped_pictures(&object_maker.objects, camera);
        if(toolbar.insert_picture) {
            load_picture_using_file_dialog(&object_maker.objects);
            toolbar.insert_picture = false;
        }
    
        BeginDrawing();
            ClearBackground(BLACK);
            BeginMode2D(camera);
                Object_draw_all(camera, &object_maker);
            EndMode2D();
            Toolbar_draw(&toolbar, &object_maker);
        EndDrawing();
    }

    for(size_t i = 0; i < object_maker.objects.size; ++i) {
        Object_free(&object_maker.objects.items[i]);
    }
    Object_array_free(&object_maker.objects);
    CloseWindow();

    return 0;
}
