#include "raylib.h"
#include <stdio.h>


int main(void) {
  InitWindow(800, 450, "árvíztűrő tükörfúrógép");
  SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);

  while(!WindowShouldClose()) {
    BeginDrawing();
      ClearBackground(BLACK);
      DrawText("The quick brown fox jumps over the lazy dog.", 190, 200, 20, WHITE);
    EndDrawing();
  }

  CloseWindow();

  return 0;
}
