#include "curve.h"
#include "toolbar.h"
#include <assert.h>


static Rectangle Curve_get_selection_rect(const Curve *curve) {
  return (Rectangle){
    .x = curve->min_x - curve->thickness,
    .y = curve->min_y - curve->thickness,
    .width = curve->max_x - curve->min_x + 2*curve->thickness,
    .height = curve->max_y - curve->min_y + 2*curve->thickness,
  };
}

static void Curve_draw(const Curve *curve) {
  if(curve->is_selected) {
    Rectangle rect = Curve_get_selection_rect(curve);
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

static void Curve_draw_new(Camera2D camera, Curve_array *curves) {
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
    if(IsMouseButtonDown(MOUSE_BUTTON_LEFT) && !Toolbar_check_collision_point(GetMousePosition())) {
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

bool Curve_is_under_mouse(Vector2 mouse_pos, const Curve *curve) {
  assert(curve);

  Rectangle rect = Curve_get_selection_rect(curve);
  if(CheckCollisionPointRec(mouse_pos, rect)) {
    for(size_t i = 0; i < curve->points.size; ++i) {
      if(CheckCollisionPointCircle(mouse_pos, curve->points.items[i], 2 * curve->thickness)) {
        return true;
      }
    }
  }

  return false;
}

void Curve_draw_all(Camera2D camera, Mode mode, Curve_array *curves) {
  assert(curves);

  if(mode == MODE_DRAW_CURVES) {
    Curve_draw_new(camera, curves);
  }

  bool is_delete_pressed = IsKeyPressed(KEY_DELETE);
  for(size_t i = 0; i < curves->size; ++i) {
    if(curves->items[i].is_selected && is_delete_pressed) {
      Curve deleted = Curve_array_delete(curves, i--); // array_delete messes with the indexes, be careful!
      Vector2_array_free(&deleted.points);
    }
    else {
        Curve_draw(&curves->items[i]);
    }
  }
  Curve_array_shrink_to_fit(curves);
}

void Curve_move(Vector2 mouse_delta, Curve *curve) {
  assert(curve);
  
  for(size_t i = 0; i < curve->points.size; ++i) {
    curve->points.items[i] = Vector2Add(curve->points.items[i], mouse_delta);
  }

  curve->min_x += mouse_delta.x;
  curve->min_y += mouse_delta.y;
  curve->max_x += mouse_delta.x;
  curve->max_y += mouse_delta.y;
}
