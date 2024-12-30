#ifndef TOOL_INCLUDED_
#define TOOL_INCLUDED_

#include "curve.h"
#include "line.h"

typedef enum {
    TOOL_KIND_SELECT,
    TOOL_KIND_CURVE,
    TOOL_KIND_LINE,
} ToolKind;

typedef struct {
    union {
        CurveTool curve_tool;
        LineTool line_tool;
    } as;
    ToolKind kind;
} Tool;

#endif //TOOL_INCLUDED_
