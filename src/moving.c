#include "moving.h"
#include "assert.h"


void update_movement(Camera2D camera, Curve_array *curves) {
  assert(curves);

  if(IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
    Vector2 mouse_pos = GetScreenToWorld2D(GetMousePosition(), camera);
    bool mouse_is_over_a_selected = false;
    for(size_t i = 0; i < curves->size; ++i) {
      if(curves->items[i].is_selected && curve_is_mouse_over(mouse_pos, &curves->items[i])) {
        mouse_is_over_a_selected = true;
        break;
      }
    }
    Vector2 mouse_delta = Vector2Scale(GetMouseDelta(), 1.0f / camera.zoom);
    for(size_t i = 0; i < curves->size; ++i) {
      if(curves->items[i].is_selected) {
        if(curves->items[i].is_moved) {
          move_curve(mouse_delta, &curves->items[i]);
        }
        else if(mouse_is_over_a_selected) {
          curves->items[i].is_moved = true;
        }
      }
    }
  }
  else {
    for(size_t i = 0; i < curves->size; ++i) {
      curves->items[i].is_moved = false;
    }
  }
}
