#include "raylib.h"
#define ELEM_TYPE Vector2
#include "array.h"
#undef ELEM_TYPE
#include <stdio.h>
#include <stdlib.h>


void draw_points(Vector2_array *points) {
  if(IsKeyPressed(KEY_SPACE)) {
    points->size = 0;
  }
  
  if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
    Vector2 mouse_pos = GetMousePosition();
    Vector2_array_push_back(points, mouse_pos);
  }

  for(size_t i = 0; i < points->size; ++i) {
    DrawCircleV(points->items[i], 5, WHITE);
  }
}


int main(void) {
  InitWindow(800, 450, "árvíztűrő tükörfúrógép");
  SetWindowState(FLAG_VSYNC_HINT | FLAG_WINDOW_RESIZABLE);

  Vector2_array points = {0};

  while(!WindowShouldClose()) {
    BeginDrawing();
      ClearBackground(BLACK);
      draw_points(&points);
    EndDrawing();
  }

  Vector2_array_free(&points);
  CloseWindow();

  return 0;
}
