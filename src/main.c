#include "raypack.h"
#include "curve.h"
#include "toolbar.h"
#include "mode.h"
#include "selection.h"
#include <stdio.h>
#include <stdlib.h>


int main(void) {
  SetWindowState(FLAG_MSAA_4X_HINT);
  InitWindow(800, 450, "PiciPaint");
  SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);

  Curve_array curves = {0};
  Camera2D camera = { .zoom = 1.0f };
  Mode mode = DRAW_CURVES;

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

    update_selection(camera, &curves);
    
    BeginDrawing();
      ClearBackground(BLACK);
      BeginMode2D(camera);
        draw_curves(camera, mode, &curves);
      EndMode2D();
      draw_toolbar(&mode);
    EndDrawing();
  }

  Curve_array_free(&curves);
  CloseWindow();

  return 0;
}
