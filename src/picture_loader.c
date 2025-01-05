#include "picture_loader.h"
#include <nfd.h>


static bool load_picture_from_file(const char *file_path, Vector2 position, Object *out_object) {
    Image image = LoadImage(file_path);
    if(IsImageValid(image)) {
        Picture picture = {
            .texture = LoadTextureFromImage(image),
            .top_left = position,
        };
        UnloadImage(image);
        *out_object = (Object){.as.picture = picture, .kind = OBJECT_KIND_PICTURE};
        return true;
    }
    else {
        return false;
    }
}

void load_dropped_pictures(Object_array *objects, Camera2D camera) {
    static const Vector2 offset = { .x = 30, .y = 30 };

    if(IsFileDropped()) {
        Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);

        FilePathList file_path_list = LoadDroppedFiles();
        for(size_t i = 0; i < file_path_list.count; ++i) {
            Object out_object;
            if(load_picture_from_file(file_path_list.paths[i], Vector2Add(mouse_pos, Vector2Scale(offset, i)), &out_object)) {
                Object_array_push_back(objects, out_object);
            }
        }

        UnloadDroppedFiles(file_path_list);
    }
}

void load_picture_using_file_dialog(Object_array *objects) {
    // source: https://github.com/raysan5/raylib/blob/master/FAQ.md#what-file-formats-are-supported-by-raylib
    static const char *supported_image_formats = "png,bmp,tga,jpg,jpeg,gif,qoi,psd,dds,hdr,ktx,astc,pkm,pvr";

    NFD_Init();

    nfdu8char_t *file_path;
    nfdu8filteritem_t filters[] = { { "Image files", supported_image_formats } }; 
    nfdopendialogu8args_t args = {0};
    args.filterList = filters;
    args.filterCount = 1;
    nfdresult_t result = NFD_OpenDialogU8_With(&file_path, &args); 
    if(result == NFD_OKAY) {
        Object out_object;
        if(load_picture_from_file(file_path, (Vector2){.x = 10, .y = 10}, &out_object)) {
            Object_array_push_back(objects, out_object);
        }
        NFD_FreePathU8(file_path);
    }
    else if(result == NFD_CANCEL) {
        // user canceled the dialog
    }
    else {
        TraceLog(LOG_ERROR, "%s", NFD_GetError());
    }

    NFD_Quit();
}
