#define ELEM_TYPE int
#include "array.h"
#undef ELEM_TYPE
#include "raylib.h"
#include <stdio.h>


int main(void) {
  int_array xs = {0};
  for(int i = 0; i < 10; ++i) {
    int_array_push_back(&xs, i);
    printf("%d: %zu, %zu\n", i, xs.size, xs.capacity);
  }
  printf("-----------\n");
  for(int i = 0; i < 5; ++i) {
    int x = int_array_pop_back(&xs);
    printf("%d\n", x);
  }
  printf("C: %zu\n", xs.capacity);
  int_array_shrink_to_fit(&xs);
  printf("C: %zu\n", xs.capacity);
  for(int i = 0; i < 5; ++i) {
    int x = int_array_pop_back(&xs);
    printf("%d\n", x);
  }
  
  return 0;
  
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
