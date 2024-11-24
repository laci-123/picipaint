#include "raylib.h"
#include "curve.h"
#include <stdio.h>
#include <stdlib.h>


int main(void) {
  InitWindow(800, 450, "árvíztűrő tükörfúrógép");
  SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);

  Curve_array curves = {0};

  while(!WindowShouldClose()) {
    BeginDrawing();
      ClearBackground(BLACK);
      draw_curves(&curves);
    EndDrawing();
  }

  Curve_array_free(&curves);
  CloseWindow();

  return 0;
}
