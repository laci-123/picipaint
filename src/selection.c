#include "selection.h"
#include "assert.h"


void Selection_update(Camera2D camera, Curve_array *curves) {
  assert(curves);

  if(IsKeyPressed(KEY_ESCAPE)) {
    for(size_t i = 0; i < curves->size; ++i) {
      curves->items[i].is_selected = false;
    }
    return;
  }

  if((IsKeyDown(KEY_LEFT_CONTROL) || IsKeyDown(KEY_RIGHT_CONTROL)) && IsKeyPressed(KEY_A)) {
    for(size_t i = 0; i < curves->size; ++i) {
      curves->items[i].is_selected = true;
    }
    return;
  }

  if(!IsMouseButtonReleased(MOUSE_BUTTON_LEFT)) {
    return;
  }

  Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
  bool any_curve_is_under_mouse = false;
  for(size_t i = 0; i < curves->size; ++i) {
    if(Curve_is_under_mouse(mouse_pos, &curves->items[i])) {
      any_curve_is_under_mouse = true;
      break;
    }
  }

  bool is_shift_down = IsKeyDown(KEY_LEFT_SHIFT) || IsKeyDown(KEY_RIGHT_SHIFT);
  for(size_t i = 0; i < curves->size; ++i) {
    if(Curve_is_under_mouse(mouse_pos, &curves->items[i])) {
      if(is_shift_down) {
        curves->items[i].is_selected = !curves->items[i].is_selected;
      }
      else {
        curves->items[i].is_selected = true;
      }
    }
    else if(!any_curve_is_under_mouse) {
      curves->items[i].is_selected = false;
    }
  }
}
