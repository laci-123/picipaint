#include "toolbar.h"
#include "assert.h"


static const int   toolbar_height = 30;
static const float max_brightness_ratio = 0.3f;
static const float height_1 = max_brightness_ratio          * toolbar_height;
static const float height_2 = (1.0f - max_brightness_ratio) * toolbar_height;
static const int   padding = 5;
static const Color base_color = DARKBLUE;
static const int font_size = 14;

bool Toolbar_check_collision_point(Vector2 point) {
  const Rectangle rect = {
    .x = 0,
    .y = 0,
    .width = GetScreenWidth(),
    .height = toolbar_height,
  };
  return CheckCollisionPointRec(point, rect);
}

typedef struct {
  char *caption;
  bool is_pressed;
} Button;

static int draw_button(int x, Button *button) {
  assert(button);
  assert(button->caption);
  
  Color background_color = ColorBrightness(base_color, 0.4f);
  const int width = MeasureText(button->caption, font_size) + 2 * padding;
  const Rectangle rect = {
    .x = x,
    .y = padding,
    .width = width,
    .height = toolbar_height - 2 * padding,
  };
  Rectangle shaddow = {
    .x = rect.x,
    .y = rect.y,
    .width = rect.width + 2,
    .height = rect.height + 2,
  };
  if(button->is_pressed) {
    shaddow.x -= 2;
    shaddow.y -= 2;
    background_color = ColorBrightness(base_color, 0.3f);
  }
  if(CheckCollisionPointRec(GetMousePosition(), rect)) {
    if(IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
      button->is_pressed = !button->is_pressed;
    }
  }
  DrawRectangleRec(shaddow, BLACK);
  DrawRectangleRec(rect, background_color);
  DrawRectangleLinesEx(rect, 1, BLACK);
  DrawText(button->caption, x + padding, 1.5 * padding, font_size, BLACK);

  return x + width;
}

void Toolbar_draw(Mode *mode) {
  const int width = GetScreenWidth();
  DrawRectangleGradientV(0, 0,        width, height_1, ColorBrightness(base_color, 0.4f), ColorBrightness(base_color, 0.5f));
  DrawRectangleGradientV(0, height_1, width, height_2, ColorBrightness(base_color, 0.5f), base_color);

  static Button button_select = (Button){ .caption = "select", .is_pressed = false };
  static Button button_draw   = (Button){ .caption = "draw", .is_pressed = true };
  int x = draw_button(10, &button_select);
  if(button_select.is_pressed) {
    button_draw.is_pressed = false;
    *mode = MODE_SELECT;
  }
  draw_button(x + 10, &button_draw);
  if(button_draw.is_pressed) {
    button_select.is_pressed = false;
    *mode = MODE_DRAW_CURVES;
  }
}
