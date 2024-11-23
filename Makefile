NAME	?= picipaint
SRC_DIR ?= ./src
BIN_DIR ?= ./bin
CC	?= gcc
CFLAGS	= -Wall -Wextra
RAYLIB	= ./raylib/src
INCLUDE = -I./raylib/src
LINK    = -lm


$(BIN_DIR)/$(NAME): $(wildcard $(SRC_DIR)/*.c) | $(RAYLIB)/libraylib.a $(BIN_DIR)
	$(CC) $? $(RAYLIB)/libraylib.a $(CFLAGS) $(INCLUDE) $(LINK) -o $(BIN_DIR)/$(NAME)

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
