#include <stdlib.h>
#include <assert.h>

#ifndef ELEM_TYPE
#error "ELEM_TYPE must be defined before including array.h."
#else

#define CONCAT_(X, Y) X##Y
#define CONCAT(X, Y) CONCAT_(X, Y)

#define ARRAY CONCAT(ELEM_TYPE, _array)

typedef struct {
    ELEM_TYPE *items;
    size_t size;
    size_t capacity; 
} ARRAY;

static inline void CONCAT(ARRAY, _push_back)(ARRAY *array, ELEM_TYPE x) {
    assert(array);
    assert(array->size <= array->capacity);

    if(array->size == array->capacity) {
        if(array->capacity == 0) {
            array->capacity = 1;
            ELEM_TYPE *new_items = malloc(array->capacity * sizeof(ELEM_TYPE));
            assert(new_items);
            array->items = new_items;
        }
        else {
            array->capacity *= 2;
            ELEM_TYPE *new_items = realloc(array->items, array->capacity * sizeof(ELEM_TYPE));
            assert(new_items);
            array->items = new_items;
        }
    }
    array->items[array->size++] = x;
}

static inline ELEM_TYPE CONCAT(ARRAY, _delete)(ARRAY *array, size_t index) {
    assert(array);
    assert(array->size > index);

    ELEM_TYPE deleted = array->items[index];
    if(array->size > 1) {
        array->items[index] = array->items[array->size - 1];
    }
    --array->size;

    return deleted;
}

static inline ELEM_TYPE *CONCAT(ARRAY, _last)(ARRAY *array) {
    assert(array);

    if(array->size > 0) {
        return &array->items[array->size - 1];
    }
    else {
        return NULL;
    }
}

static inline const ELEM_TYPE *CONCAT(ARRAY, _last_const)(const ARRAY *array) {
    assert(array);

    if(array->size > 0) {
        return &array->items[array->size - 1];
    }
    else {
        return NULL;
    }
}

static inline void CONCAT(ARRAY, _shrink_to_fit)(ARRAY *array) {
    assert(array);
    assert(array->size <= array->capacity);

    array->capacity = array->size;
    if(array->capacity > 0) {
        ELEM_TYPE *new_items = realloc(array->items, array->capacity * sizeof(ELEM_TYPE));
        assert(new_items);
        array->items = new_items;
    }
    else {
        free(array->items);
        array->items = NULL;
    }
}

static inline void CONCAT(ARRAY, _free)(ARRAY *array) {
    assert(array);

    array->size = 0;
    array->capacity = 0;
    free(array->items);
    array->items = NULL;
}

#endif //ifndef ELEM_TYPE
