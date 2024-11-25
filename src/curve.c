#include "curve.h"

static void draw_curve(const Curve *curve) {
  for(size_t i = 0; i < curve->points.size; ++i) {
    if(i > 0) {
      Vector2 begin = curve->points.items[i - 1];
      Vector2 end   = curve->points.items[i];
      DrawLineEx(begin, end, curve->thickness, curve->color);
    }
  }
}

void draw_curves(Camera2D camera, Curve_array *curves) {
  static bool pen_down = false;
  if(pen_down) {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
      Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
      Vector2_array_push_back(&curves->items[curves->size - 1].points, mouse_pos);
    }
    else {
      pen_down = false;
    }
  }
  else {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
      pen_down = true;
      Curve_array_push_back(curves, (Curve){.points = {0}, .color = WHITE, .thickness = 5.0f});
    }
  }
  for(size_t i = 0; i < curves->size; ++i) {
    draw_curve(&curves->items[i]);
  }
}
