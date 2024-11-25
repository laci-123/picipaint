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
  if(array->size == array->capacity) {
    if(array->capacity == 0) {
      array->capacity = 1;
      ELEM_TYPE *new_items = malloc(array->capacity * sizeof(ELEM_TYPE));
      assert(new_items != NULL);
      array->items = new_items;
    }
    else {
      array->capacity *= 2;
      ELEM_TYPE *new_items = realloc(array->items, array->capacity * sizeof(ELEM_TYPE));
      assert(new_items != NULL);
      array->items = new_items;
    }
  }
  array->items[array->size++] = x;
}

static inline ELEM_TYPE CONCAT(ARRAY, _pop_back)(ARRAY *array) {
  assert(array->size > 0);
  return array->items[--array->size];
}

static inline ELEM_TYPE *CONCAT(ARRAY, _last)(ARRAY *array) {
  if(array->size > 0) {
    return &array->items[array->size - 1];
  }
  else {
    return NULL;
  }
}

static inline const ELEM_TYPE *CONCAT(ARRAY, _last_const)(const ARRAY *array) {
  if(array->size > 0) {
    return &array->items[array->size - 1];
  }
  else {
    return NULL;
  }
}

static inline void CONCAT(ARRAY, _shrink_to_fit)(ARRAY *array) {
  array->capacity = array->size;
  ELEM_TYPE *new_items = realloc(array->items, array->capacity * sizeof(ELEM_TYPE));
  assert(new_items != NULL);
  array->items = new_items;
}

static inline void CONCAT(ARRAY, _free)(ARRAY *array) {
  array->size = 0;
  array->capacity = 0;
  free(array->items);
  array->items = NULL;
}

#endif //ifndef ELEM_TYPE
