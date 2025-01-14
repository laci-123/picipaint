#ifndef SELECTABLE_INCLUDED_
#define SELECTABLE_INCLUDED_


typedef enum {
    RESIZE_NONE,
    RESIZE_TOP,
    RESIZE_BOTTOM,
    RESIZE_LEFT,
    RESIZE_RIGHT,
    RESIZE_TOP_LEFT,
    RESIZE_TOP_RIGHT,
    RESIZE_BOTTOM_LEFT,
    RESIZE_BOTTOM_RIGHT,
} Resize;

typedef struct {
    Resize resize;
    bool is_selected;
    bool is_moved;
} Selectable;

#endif //SELECTABLE_INCLUDED_
