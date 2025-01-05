NAME	?= picipaint
SRC_DIR ?= ./src
BIN_DIR ?= ./bin
CC		?= gcc
CFLAGS	= -Wall -Wextra -std=c23 -pedantic
RAYLIB	= ./raylib/src
NFD     = ./nativefiledialog-extended
INCLUDE = -I./raylib/src -I./nativefiledialog-extended/src/include
LINK    = `pkg-config --libs gtk+-3.0` -lm
C_FILES = $(wildcard $(SRC_DIR)/*.c)
H_FILES = $(wildcard $(SRC_DIR)/*.h)


$(BIN_DIR)/$(NAME): $(C_FILES) $(H_FILES) | $(RAYLIB)/libraylib.a $(NFD)/build/src/libnfd.a $(BIN_DIR)
	$(CC) $(C_FILES) $(RAYLIB)/libraylib.a $(NFD)/build/src/libnfd.a $(CFLAGS) $(INCLUDE) $(LINK) -o $(BIN_DIR)/$(NAME)

$(RAYLIB)/libraylib.a: 
	cd $(RAYLIB) && $(MAKE)

$(NFD)/build/src/libnfd.a:
	cd $(NFD) && \
	mkdir build && \
	cd build && \
	cmake -DCMAKE_BUILD_TYPE=Release -DNFD_BUILD_TESTS=OFF .. && \
	cmake --build .

$(BIN_DIR):
	mkdir $(BIN_DIR)


.PHONY: run clean clean_all

run: $(BIN_DIR)/$(NAME)
	./$(BIN_DIR)/$(NAME)

clean:
	rm -rf $(BIN_DIR)

clean_all: clean
	cd $(RAYLIB) && $(MAKE) clean
	rm -rf $(NFD)/build
