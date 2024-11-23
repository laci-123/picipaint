NAME	?= picipaint
SRC_DIR ?= ./src
BIN_DIR ?= ./bin
CC	?= gcc
CFLAGS	= -Wall -Wextra
RAYLIB	= ./raylib/src
INCLUDE = -I./raylib/src
LINK    = -lm
C_FILES = $(wildcard $(SRC_DIR)/*.c)
H_FILES = $(wildcard $(SRC_DIR)/*.h)


$(BIN_DIR)/$(NAME): $(C_FILES) $(H_FILES) | $(RAYLIB)/libraylib.a $(BIN_DIR)
	$(CC) $(C_FILES) $(RAYLIB)/libraylib.a $(CFLAGS) $(INCLUDE) $(LINK) -o $(BIN_DIR)/$(NAME)

$(RAYLIB)/libraylib.a: 
	cd $(RAYLIB) && $(MAKE)

$(BIN_DIR):
	mkdir $(BIN_DIR)


.PHONY: run clean clean_all

run: $(BIN_DIR)/$(NAME)
	./$(BIN_DIR)/$(NAME)

clean:
	rm -rf $(BIN_DIR)

clean_all: clean
	cd $(RAYLIB) && $(MAKE) clean
