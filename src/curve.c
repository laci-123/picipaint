#include "curve.h"
#include "toolbar.h"
#include <assert.h>


bool curve_is_mouse_over(Vector2 mouse_pos, const Curve *curve) {
  Rectangle rect = {
    .x = curve->min_x - curve->thickness,
    .y = curve->min_y - curve->thickness,
    .width = curve->max_x - curve->min_x + 2*curve->thickness,
    .height = curve->max_y - curve->min_y + 2*curve->thickness,
  };

  if(CheckCollisionPointRec(mouse_pos, rect)) {
    for(size_t i = 0; i < curve->points.size; ++i) {
      if(CheckCollisionPointCircle(mouse_pos, curve->points.items[i], 2 * curve->thickness)) {
        return true;
      }
    }
  }

  return false;
}

static void draw_curve(const Curve *curve) {
  assert(curve);
  
  if(curve->is_selected) {
    Rectangle rect = {
        .x = curve->min_x - curve->thickness,
        .y = curve->min_y - curve->thickness,
        .width = curve->max_x - curve->min_x + 2*curve->thickness,
        .height = curve->max_y - curve->min_y + 2*curve->thickness,
    };
    DrawRectangleLinesEx(rect, 1.0f, WHITE);
  }

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

static void draw_new_curve(Camera2D camera, Curve_array *curves) {
  static bool pen_down = false;
  if(pen_down) {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
      Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
      Curve *last_curve = Curve_array_last(curves);
      assert(last_curve && "`curves` has at least one element, because when the pen was put down we added one.");
      const Vector2 *last_point = Vector2_array_last_const(&last_curve->points);
      if(!(last_point && Vector2Equals(*last_point, mouse_pos))) {
        if(mouse_pos.x < last_curve->min_x) {
          last_curve->min_x = mouse_pos.x;
        }
        if(mouse_pos.x > last_curve->max_x) {
          last_curve->max_x = mouse_pos.x;
        }
        if(mouse_pos.y < last_curve->min_y) {
          last_curve->min_y = mouse_pos.y;
        }
        if(mouse_pos.y > last_curve->max_y) {
          last_curve->max_y = mouse_pos.y;
        }
        Vector2_array_push_back(&last_curve->points, mouse_pos);
      }
    }
    else {
      pen_down = false;
    }
  }
  else {
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT) && !check_collision_point_toolbar(GetMousePosition())) {
      pen_down = true;

      Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
      Curve_array_push_back(curves, (Curve){
          .points = {0},
          .color = WHITE,
          .thickness = 5.0f,
          .min_x = mouse_pos.x,
          .max_x = mouse_pos.x,
          .min_y = mouse_pos.y,
          .max_y = mouse_pos.y,
        });
    }
  }
}

void draw_curves(Camera2D camera, Mode mode, Curve_array *curves) {
  assert(curves);

  if(mode == DRAW_CURVES) {
    draw_new_curve(camera, curves);
  }

  for(size_t i = 0; i < curves->size; ++i) {
    draw_curve( &curves->items[i]);
  }
}
