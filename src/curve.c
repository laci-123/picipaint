#include "curve.h"
#include <assert.h>


static void draw_curve(const Curve *curve) {
  if(curve->points.size == 1) {
    DrawCircleV(curve->points.items[0], curve->thickness * 0.6f, curve->color);
  }
  else {
    for(size_t i = 0; i < curve->points.size; ++i) {
        if(i > 0) {
        Vector2 begin = curve->points.items[i - 1];
        Vector2 end   = curve->points.items[i];
        DrawLineEx(begin, end, curve->thickness, curve->color);
        }
    }
  }
}

void draw_curves(Camera2D camera, Curve_array *curves) {
  static bool pen_down = false;
  if(pen_down) {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
      Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
      Curve *last_curve = Curve_array_last(curves);
      assert(last_curve && "`curves` has at least one element, because when the pen was put down we added one.");
      const Vector2 *last_point = Vector2_array_last_const(&last_curve->points);
      if(!(last_point && Vector2Equals(*last_point, mouse_pos))) {
        Vector2_array_push_back(&last_curve->points, mouse_pos);
      }
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
