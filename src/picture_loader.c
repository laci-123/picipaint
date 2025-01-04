#include "picture_loader.h"


static const Vector2 offset = { .x = 30, .y = 30 };


void load_dropped_pictures(Object_array *objects, Camera2D camera) {
    if(IsFileDropped()) {
        Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);

        FilePathList file_path_list = LoadDroppedFiles();
        for(size_t i = 0; i < file_path_list.count; ++i) {
            Image image = LoadImage(file_path_list.paths[i]);
            if(!IsImageValid(image)) {
                continue;
            }

            Picture picture = {
                .texture = LoadTextureFromImage(image),
                .top_left = Vector2Add(mouse_pos, Vector2Scale(offset, i)),
            };
            UnloadImage(image);
            Object_array_push_back(objects, (Object){.as.picture = picture, .kind = OBJECT_KIND_PICTURE});
        }
        UnloadDroppedFiles(file_path_list);
    }
}
