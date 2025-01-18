#ifndef SELECTABLE_INCLUDED_
#define SELECTABLE_INCLUDED_


typedef enum {
    RESIZE_NONE   = 0,
    RESIZE_TOP    = 1,
    RESIZE_BOTTOM = 2,
    RESIZE_LEFT   = 4,
    RESIZE_RIGHT  = 8,
} Resize;

typedef struct {
    Resize resize;
    bool is_selected;
    bool is_moved;
} Selectable;

#endif //SELECTABLE_INCLUDED_
